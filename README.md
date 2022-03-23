# ffzf
Fast fuzzy string matching for Python. 

# Installation 
```
pip install ffzf
```

# Usage
```python
# Find closest string matching
from ffzf import closest
best_match = closest("hello", ["harps", "apples", "jello"])

# Find n best matches
from ffzf import n_closest
best_matches = n_closest("hello", ["harps", "apples", "jello"], 2)

from ffzf import JAROWINKLER
# Specify an algorithm (default is levenshtein distance)
best_match = closest("hello", ["harps", "apples", "jello"], algorithm=JAROWINKLER)

# Call algorithm directly
from ffzf import levenshtein_distance
dist = levenshtein_distance("hello", "jello")

# Case sensitive comparison (default is case insensitive)
dist = levenshtein_distance("Hello", "hello", case_sensitive=True)
best_match = closest("Hello", ["harps", "apples", "jello"], case_sensitive=True)

# Remove whitespace (default is to keep the whitespace in strings)
dist = levenshtein_distance("hello world", "helloworld", remove_whitespace=True)
```

# Supported Algorithms
- Levenshtein Distance (default)
- Jaro Similarity ("JARO")
- Jaro-Winkler Similarity ("JAROWINKLER")
- Hamming Distance ("HAMMING")
<br><br><br>
![workflow](https://github.com/addisonc6/ffzf/actions/workflows/CI.yml/badge.svg)