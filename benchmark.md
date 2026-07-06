# Benchmark – Finding Borders

## Objective

The objective is to find all border lengths of a string. A border is a prefix that is also a suffix of the string, excluding the entire string itself.

Two algorithms are compared:

1. Knuth–Morris–Pratt (KMP) Prefix Function
2. Polynomial Rolling Hash

---

## Algorithm 1 – KMP Prefix Function

The KMP algorithm computes the prefix function for the string. The last value of the prefix function indicates the longest border, and repeatedly following previous prefix values reveals all shorter borders.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Deterministic algorithm.
- Guaranteed linear running time.
- No possibility of collisions.

### Disadvantages

- Slightly harder to understand.
- Requires knowledge of prefix-function computation.

---

## Algorithm 2 – Polynomial Rolling Hash

The rolling hash algorithm computes hash values for all prefixes. It compares the hash of each prefix with the corresponding suffix hash. Equal hashes indicate candidate borders.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Very flexible.
- Widely used in string algorithms.
- Naturally demonstrates hashing techniques.

### Disadvantages

- Hash collisions are theoretically possible.
- Requires selecting suitable hash parameters.

---

## Comparison

| Feature | KMP | Rolling Hash |
|---------|-----|--------------|
| Time Complexity | O(n) | O(n) |
| Extra Space | O(n) | O(n) |
| Deterministic | Yes | No (small collision risk) |
| Uses Hashing | No | Yes |
| Uses Prefix Function | Yes | No |

---

## Conclusion

Both algorithms solve the Finding Borders problem in linear time and satisfy the CSES constraints.

The KMP algorithm provides an exact deterministic solution using the prefix function and guarantees correct results without collisions. The Rolling Hash algorithm applies polynomial hashing to compare prefixes and suffixes efficiently, demonstrating the use of hashing techniques from the course syllabus.

For this problem, KMP is generally preferred when absolute correctness is required, while Rolling Hash is valued for its simplicity and broad applicability in string-processing problems.
