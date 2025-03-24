#![deny(clippy::unwrap_used, clippy::expect_used)]
use rand::{Rng, thread_rng};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::error::HashError;

pub struct PasswordGenerator;

impl PasswordGenerator {
    pub fn generate_passwords(
        num: usize,
        chars: u8,
        threads: usize,
    ) -> Result<Vec<String>, HashError> {
        let passwords = Arc::new(Mutex::new(Vec::with_capacity(num)));
        let mut handles = vec![];
        let error_flag = Arc::new(Mutex::new(None));

        let passwords_per_thread = num / threads;
        let remainder = num % threads;

        for i in 0..threads {
            let thread_passwords = Arc::clone(&passwords);
            let thread_error = Arc::clone(&error_flag);
            let mut thread_password_count = passwords_per_thread;
            if i == 0 {
                thread_password_count += remainder;
            }

            let handle = thread::spawn(move || {
                let mut rng = thread_rng();
                let mut local_passwords = Vec::with_capacity(thread_password_count);

                for _ in 0..thread_password_count {
                    let password: String = (0..chars)
                        .map(|_| {
                            // ASCII printable characters
                            let char_code = rng.gen_range(32..=126);
                            char::from(char_code)
                        })
                        .collect();
                    local_passwords.push(password);
                }

                if let Ok(mut passwords) = thread_passwords.lock() {
                    passwords.extend(local_passwords);
                } else {
                    let mut err = thread_error.lock().map_err(|_| {
                        HashError::HashingFailed("Failed to lock error flag".to_string())
                    })?;
                    *err = Some(HashError::HashingFailed(
                        "Failed to lock passwords".to_string(),
                    ));
                }
                Ok::<(), HashError>(())
            });

            handles.push(handle);
        }

        // make sure we have handle for  each one
        for handle in handles {
            if let Err(e) = handle.join() {
                return Err(HashError::HashingFailed(format!(
                    "Thread panicked: {:?}",
                    e
                )));
            }
        }

        // Check if any thread encountered an error
        if let Ok(err) = error_flag.lock() {
            if let Some(e) = &*err {
                return Err(e.clone());
            }
        } else {
            return Err(HashError::HashingFailed(
                "Failed to check for thread errors".to_string(),
            ));
        }

        // Move out of the Arc and Mutex
        match Arc::try_unwrap(passwords) {
            Ok(mutex) => match mutex.into_inner() {
                Ok(result) => Ok(result),
                Err(_) => Err(HashError::HashingFailed(
                    "Couldnt to unwrap mutex".to_string(),
                )),
            },
            Err(_) => Err(HashError::HashingFailed(
                "Erroing during unwrap Arc".to_string(),
            )),
        }
    }
}
