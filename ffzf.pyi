def closest(
    target: str, 
    candidates: list[str], 
    algorithm: str = "levenshtein", 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> str:
    """
    Find the closest match to the target string in the list of candidates.
    :param target: The target string to find a match for.
    :param candidates: The list of strings to find a match in.
    :param algorithm: The algorithm to use for finding the closest match. Options are:
        - "levenshtein"
        - "jaro"
        - "jarowinkler"
        - "hamming"
    :param case_sensitive: Whether or not to use case sensitivity when finding the closest match.
    :param remove_whitespace: Whether or not to remove whitespace when finding the closest match.
    """
    ...


def n_closest(
    target: str, 
    candidates: list[str], 
    n: int, algorithm: str = "levenshtein", 
    case_senstive: bool = False, 
    remove_whitespace: bool = False) -> list[str]:
    """
    Find the n closest matches to the target string in the list of candidates.
    :param target: The target string to find a match for.
    :param candidates: The list of strings to find a match in.
    :param n: The number of closest matches to return.
    :param algorithm: The algorithm to use for finding the closest match. Options are:
        - "levenshtein"
        - "jaro"
        - "jarowinkler"
        - "hamming"
    :param case_sensitive: Whether or not to use case sensitivity when finding the closest matches.
    :param remove_whitespace: Whether or not to remove whitespace when finding the closest matches.
    """
    ...


def closest_index_pair(
    target: str, 
    text: str, 
    algorithm: str = "levenshtein", 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> tuple[int, int]:
    """
    Find the the start and end index of the closest match to the target in the text.
    :param target: The target string to find a match for.
    :param text: The list of strings to find a match in.
    :param algorithm: The algorithm to use for finding the closest match. Options are:
        - "levenshtein"
        - "jaro"
        - "jarowinkler"
        - "hamming"
    :param case_sensitive: Whether or not to use case sensitivity when finding the closest match.
    :param remove_whitespace: Whether or not to remove whitespace when finding the closest match.
    """
    ...

def levenshtein_distance(
    a: str, 
    b: str, 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> int:
    """
    Calculate the Levenshtein distance between two strings.
    :param a: The first string to compare.
    :param b: The second string to compare.
    :param case_sensitive: Whether or not to use case sensitivity when calculating the Levenshtein distance.
    :param remove_whitespace: Whether or not to remove whitespace when calculating the Levenshtein distance.
    """
    ...

def jaro_similarity(
    a: str, 
    b: str, 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> float:
    """
    Calculate the Jaro similarity between two strings.
    :param a: The first string to compare.
    :param b: The second string to compare.
    :param case_sensitive: Whether or not to use case sensitivity when calculating the Jaro similarity.
    :param remove_whitespace: Whether or not to remove whitespace when calculating the Jaro similarity.
    """
    ...

def jaro_winkler_similarity(
    a: str, 
    b: str, 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> float:
    """
    Calculate the Jaro-Winkler similarity between two strings.
    :param a: The first string to compare.
    :param b: The second string to compare.
    :param case_sensitive: Whether or not to use case sensitivity when calculating the Jaro-Winkler similarity.
    :param remove_whitespace: Whether or not to remove whitespace when calculating the Jaro-Winkler similarity.
    """
    ...

def hamming_distance(
    a: str, 
    b: str, 
    case_sensitive: bool = False, 
    remove_whitespace: bool = False) -> int:
    """
    Calculate the Hamming distance between two strings.
    :param a: The first string to compare.
    :param b: The second string to compare.
    :param case_sensitive: Whether or not to use case sensitivity when calculating the Hamming distance.
    :param remove_whitespace: Whether or not to remove whitespace when calculating the Hamming distance.
    """
    ...
