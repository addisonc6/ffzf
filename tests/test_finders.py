import unittest

from ffzf import closest, n_closest, closest_index_pair, closest_with_score, n_closest_with_score


class TestFindingFunctions(unittest.TestCase):

    def test_closest(self):
        self.assertEqual(closest("hello", ["hello", "world"]), "hello")
        self.assertEqual(closest("hello", ["world", "hello"]), "hello")
        self.assertEqual(closest("hello", ["YELLO", "hey there"]), "YELLO")
        self.assertEqual(
            closest("travel", ["gravel", "gambit", "gated"], algorithm="jaro"), "gravel")
        self.assertEqual(closest(
            "travel", ["gravel", "gambit", "gated"], algorithm="jarowinkler"), "gravel")
        self.assertEqual(closest(
            "travel", ["gravel", "gambit", "guards"], algorithm="hamming"), "gravel")
        with self.assertRaises(ValueError):
            closest("travel", ["gravel", "gambit",
                    "gated"], algorithm="unknown")
        with self.assertRaises(ValueError):
            closest("travel", ["gravel", "gambit",
                    "gated"], algorithm="hamming")
        with self.assertRaises(ValueError):
            closest("travel", [])

    def test_n_closest(self):
        self.assertEqual(n_closest(
            "hello", ["yello", "jello", "harps", "languid"], n=2), ["yello", "jello"])
        self.assertEqual(n_closest("hello", ["yello", "jello", "harps", "languid"], n=3), [
                         "yello", "jello", "harps"])
        self.assertEqual(n_closest("hello", [
                         "yello", "jello", "harps", "languid"], n=3, algorithm="jaro"), ["yello", "jello", "harps"])
        self.assertEqual(n_closest("hello", [
                         "yello", "jello", "harps", "languid"], n=3, algorithm="jarowinkler"), ["yello", "jello", "harps"])
        with self.assertRaises(ValueError):
            n_closest("travel", ["gravel", "gambit",
                      "gated"], n=2, algorithm="unknown")
        with self.assertRaises(ValueError):
            n_closest("travel", ["gravel", "gambit",
                      "gated"], n=2, algorithm="hamming")
        with self.assertRaises(ValueError):
            n_closest("travel", [], n=2)
        with self.assertRaises(ValueError):
            n_closest("travel", ["train", "tracks", "towered"], n=0)
        with self.assertRaises(ValueError):
            n_closest("travel", ["train", "tracks", "towered"], n=10)

    def test_closest_index_pair(self):
        self.assertEqual(closest_index_pair("hello", "hello world"), (0, 5))
        self.assertEqual(closest_index_pair("hello", "world hello"), (6, 11))
        self.assertEqual(closest_index_pair("hello", "YELLO there"), (0, 5))
        self.assertEqual(closest_index_pair(
            "travel", "gravel gambit gated", algorithm="jaro"), (0, 6))
        self.assertEqual(closest_index_pair(
            "travel", "gravel gambit gated", algorithm="jarowinkler"), (0, 6))
        self.assertEqual(closest_index_pair(
            "travel", "gravel gambit guards", algorithm="hamming"), (0, 6))
        with self.assertRaises(ValueError):
            closest_index_pair(
                "travel", "gravel gambit gated", algorithm="unknown")

    def test_closest_with_score(self):
        self.assertEqual(closest_with_score("euphoria", ["excitement", "elation", "joyful"]), ("elation", 7))
    
    def test_n_closest_with_score(self):
        self.assertEqual(n_closest_with_score("euphoria", ["excitement", "elation", "joyful"], n=2), [("elation", 7), ("joyful", 8)])


if __name__ == '__main__':
    unittest.main()
