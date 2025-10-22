#!/usr/bin/env python3
"""
Sprint 0 Tracer Bullet - Python Test Case

This is the Python source code that will be transpiled.
Target: Prove that Python len() → C list_length() → Rust Vec::len()
"""

def my_len(x):
    """Return the length of x using built-in len()"""
    return len(x)


if __name__ == "__main__":
    # Test the function
    test_list = [1, 2, 3, 4, 5]
    result = my_len(test_list)
    print(f"Length of {test_list} is {result}")
    assert result == 5, f"Expected 5, got {result}"
    print("✅ Python test passed")
