#![deny(clippy::unwrap_used, clippy::expect_used)]
use std::sync::{Arc, Mutex};
use std::thread;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, create_hasher};

pub struct ThreadedHashGenerator {
    algorithm: HashAlgorithm,
    threads: usize,
}

impl ThreadedHashGenerator {
    pub fn new(algorithm: HashAlgorithm, threads: usize) -> Self {
        ThreadedHashGenerator { algorithm, threads }
    }

    pub fn generate_hashes(&self, passwords: Vec<String>) -> Result<Vec<Vec<u8>>, HashError> {
        let hashes = Arc::new(Mutex::new(Vec::with_capacity(passwords.len())));
        let mut handles = vec![];
        let error_flag = Arc::new(Mutex::new(None));

        let passwords_per_thread = passwords.len() / self.threads;
        let remainder = passwords.len() % self.threads;

        let passwords = Arc::new(passwords);

        for i in 0..self.threads {
            let thread_hashes = Arc::clone(&hashes);
            let thread_passwords = Arc::clone(&passwords);
            let thread_error = Arc::clone(&error_flag);
            let algorithm = self.algorithm;

            let start_idx = i * passwords_per_thread;
            let mut end_idx = start_idx + passwords_per_thread;
            if i == 0 {
                end_idx += remainder;
            }

            let handle = thread::spawn(move || -> Result<(), HashError> {
                let hasher = create_hasher(algorithm).map_err(|e| {
                    HashError::HashingFailed(format!("Failed to create hasher: {}", e))
                })?;

                let mut local_hashes = Vec::with_capacity(end_idx - start_idx);

                for password in thread_passwords[start_idx..end_idx].iter() {
                    match hasher.generate_hash(password) {
                        Ok(hash) => local_hashes.push(hash),
                        Err(e) => {
                            let mut err_guard = thread_error.lock().map_err(|_| {
                                HashError::HashingFailed("Failed to lock error flag".to_string())
                            })?;
                            *err_guard = Some(e.clone());
                            return Err(HashError::HashingFailed(format!(
                                "Error hashing password: {}",
                                e
                            )));
                        }
                    }
                }

                let mut hashes_guard = thread_hashes
                    .lock()
                    .map_err(|_| HashError::HashingFailed("Failed to lock hashes".to_string()))?;
                // shared collection for local hashes

                hashes_guard.extend(local_hashes);
                Ok(())
            });

            handles.push(handle);
        }

        for handle in handles {
            match handle.join() {
                Ok(thread_result) => {
                    thread_result?;
                }
                Err(_) => return Err(HashError::HashingFailed("Thread panicked".to_string())),
            }
        }

        if let Ok(err_guard) = error_flag.lock() {
            if let Some(e) = &*err_guard {
                return Err(e.clone());
            }
        } else {
            return Err(HashError::HashingFailed(
                "crashed during checking for thread errors".to_string(),
            ));
        }

        match Arc::try_unwrap(hashes) {
            Ok(mutex) => match mutex.into_inner() {
                Ok(result) => Ok(result),
                Err(_) => Err(HashError::HashingFailed(
                    "Not able to unwrap mutex".to_string(),
                )),
            },
            Err(_) => Err(HashError::HashingFailed("Failed to unwrap Arc".to_string())),
        }
    }
}
