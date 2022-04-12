from os import remove
import random
from functools import partial
from timeit import default_timer as timer

import matplotlib.pyplot as plt
from ffzf import closest_with_score, n_closest_with_score
from fuzzywuzzy import process as fuzzywuzzy_process
from rapidfuzz import process as rapidfuzz_process
import rapidfuzz
from tqdm import tqdm

CANTERBURY_TALES = open('benches/canterbury.txt').read().split(' ')


def random_letter():
    if random.randint(0, 1):
        return chr(random.randint(ord('a'), ord('z')))
    else:
        return chr(random.randint(ord('A'), ord('Z')))


def generate_sample_words():
    sample_words = set()
    for n in range(1, 19):
        sample_words.add(''.join(random_letter() for _ in range(n)))
    return sample_words


sample_words = generate_sample_words()

closest_scorers = [
    (partial(closest_with_score, remvoe_whitespace=True), "ffzf"),
    (partial(rapidfuzz_process.extractOne,
     scorer=rapidfuzz.string_metric.levenshtein), "rapidfuzz"),
    (fuzzywuzzy_process.extractOne, "fuzzywuzzy")
]

n_equal_10_closest_scorers = [
    (partial(n_closest_with_score, n=10, remove_whitespace=True), "ffzf"),
    (partial(rapidfuzz_process.extract, limit=10,
     scorer=rapidfuzz.string_metric.levenshtein), "rapidfuzz"),
    (partial(fuzzywuzzy_process.extract, limit=10), "fuzzywuzzy")
]

n_equal_100_closest_scorers = [
    (partial(n_closest_with_score, n=100, remove_whitespace=True), "ffzf"),
    (partial(rapidfuzz_process.extract, limit=100,
     scorer=rapidfuzz.string_metric.levenshtein), "rapidfuzz"),
    (partial(fuzzywuzzy_process.extract, limit=100), "fuzzywuzzy")
]

time_to_extract_closest = {
    'ffzf': 0,
    'rapidfuzz': 0,
    'fuzzywuzzy': 0,
}

time_to_extract_10_closest = {
    'ffzf': 0,
    'rapidfuzz': 0,
    'fuzzywuzzy': 0,
}

time_to_extract_100_closest = {
    'ffzf': 0,
    'rapidfuzz': 0,
    'fuzzywuzzy': 0,
}


def benchmark_closest():
    for library_scorer in closest_scorers:
        print(f"Benchmarking {library_scorer[1]}")
        time = timer()
        for word in tqdm(sample_words, position=0, leave=True):
            library_scorer[0](word, CANTERBURY_TALES)
        time_to_extract_closest[library_scorer[1]] += timer() - time


def benchmark_10_closest():
    for library_scorer in n_equal_10_closest_scorers:
        print(f"Benchmarking {library_scorer[1]}")
        time = timer()
        for word in tqdm(sample_words, position=0, leave=True):
            library_scorer[0](word, CANTERBURY_TALES)
        time_to_extract_10_closest[library_scorer[1]] += timer() - time


def benchmark_100_closest():
    for library_scorer in n_equal_100_closest_scorers:
        print(f"Benchmarking {library_scorer[1]}")
        time = timer()
        for word in tqdm(sample_words, position=0, leave=True):
            library_scorer[0](word, CANTERBURY_TALES)
        time_to_extract_100_closest[library_scorer[1]] += timer() - time


def graph_results_of_closest():
    plt.bar(range(len(time_to_extract_closest)),
            time_to_extract_closest.values(), align='center')
    plt.ylim(0, 5)
    plt.ylabel('Time to extract closest (seconds)')
    plt.xlabel('Library')
    plt.xticks(range(len(time_to_extract_closest)),
               time_to_extract_closest.keys())
    plt.savefig('closest_benchmark.png')


def graph_results_of_10_closest():
    plt.bar(range(len(time_to_extract_10_closest)),
            time_to_extract_10_closest.values(), align='center')
    plt.ylim(0, 5)
    plt.ylabel('Time to extract 10 closest (seconds)')
    plt.xlabel('Library')
    plt.xticks(range(len(time_to_extract_10_closest)),
               time_to_extract_10_closest.keys())
    plt.savefig('10_closest_benchmark.png')


def graph_results_of_100_closest():
    plt.bar(range(len(time_to_extract_100_closest)),
            time_to_extract_100_closest.values(), align='center')
    plt.ylim(0, 5)
    plt.ylabel('Time to extract 100 closest (seconds)')
    plt.xlabel('Library')
    plt.xticks(range(len(time_to_extract_100_closest)),
               time_to_extract_100_closest.keys())
    plt.savefig('100_closest_benchmark.png')


if __name__ == '__main__':
    benchmark_closest()
    benchmark_10_closest()
    benchmark_100_closest()
    print(f"Time to extract closest: {time_to_extract_closest}")
    print(f"Time to extract 10 closest: {time_to_extract_10_closest}")
    print(f"Time to extract 100 closest: {time_to_extract_100_closest}")
    graph_results_of_closest()
    graph_results_of_10_closest()
    graph_results_of_100_closest()
