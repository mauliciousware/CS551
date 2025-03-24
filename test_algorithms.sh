#!/bin/bash
# TESTS

echo "===== HASHASSIN proj 1 test script ====="

#as per reqs, make sure binary name is correct
BINARY="./target/debug/hashassin"

echo "First Step 1: Creating test passwords:"
$BINARY gen-passwords --num 5 --chars 8 --threads 2 --out-file test_passwords.txt

echo -e "\n2: Testing all hash algorithms:"

# our suported algorithms to test
algorithms=("md5" "sha256" "sha3_512" "scrypt" "sha512" "blake3" "argon2")

# run out command for each algorithm
for algo in "${algorithms[@]}"; do
  echo -e "\n- Testing $algo algorithm -"
  
  $BINARY gen-hashes --in-file test_passwords.txt --out-file "${algo}_test.hash" --algorithm "$algo" --threads 2
  
  if [ $? -ne 0 ]; then
    echo "Failure: Failed to generate hashes with $algo algorithm"
    continue
  fi
  
  output=$($BINARY dump-hashes --in-file "${algo}_test.hash")
  
  if [ $? -ne 0 ]; then
    echo "Error: Failed to dump hashes for $algo algorithm"
    continue
  fi
  
  echo "$output"
  
  # VERSION line
  if ! echo "$output" | grep -q "VERSION: 1"; then
    echo "Test fail: invalid version in output for $algo"
    continue
  fi
  
  # ALGORITHM line
  if ! echo "$output" | grep -q "ALGORITHM: $algo"; then
    echo "Test fail: Invalid algorithm name in output for $algo"
    continue
  fi
  
  # PASSWORD LENGTH line
  if ! echo "$output" | grep -q "PASSWORD LENGTH: 8"; then
    echo "ERROR: invalid password length in output for $algo"
    continue
  fi
  
  # # of of hashes
  hash_count=$(echo "$output" | tail -n +4 | wc -l)
  if [ "$hash_count" -ne 5 ]; then
    echo "ERROR: Expected 5 hashes, found $hash_count for $algo"
    continue
  fi
  
  echo "$algo algorithm passed tests"
done

echo -e "\n#3 - Testing error handling"

$BINARY gen-hashes --in-file nonexistent.txt --out-file error_test.hash --algorithm md5
$BINARY gen-hashes --in-file test_passwords.txt --out-file error_test.hash --algorithm invalid_algo

# with mixed length passwords
echo -e "pass1\npassword2\npass3" > mixed_passwords.txt
$BINARY gen-hashes --in-file mixed_passwords.txt --out-file error_test.hash --algorithm md5

echo -e "\n#4: Testing truncate and overwrite functionality"

# test create a file that should be overwritten
echo "Generating passwords to overwrite file"
$BINARY gen-passwords --num 3 --chars 6 --out-file overwrite_test.txt
echo "Content after overwrite:"
cat overwrite_test.txt

# Test stdout when --out-file is not specified
echo -e "\n5: Testing output to stdout when --out-file isn't specified"
$BINARY gen-passwords --num 3 --chars 4

# Test char validation
echo -e "\n6:test character validation"
$BINARY gen-passwords --num 5 --chars 0 --out-file /dev/null
$BINARY gen-passwords --num 1 --chars 255 --out-file large_char_test.txt
cat large_char_test.txt

echo -e "\n Done"