import unittest

from ffzf import closest, n_closest

class TestFindingFunctions(unittest.TestCase):

    def test_closest(self):
        self.assertEqual(closest("hello", ["hello", "world"]), "hello")
        self.assertEqual(closest("hello", ["world", "hello"]), "hello")
        self.assertEqual(closest("hello", ["YELLO", "hey there"]), "YELLO")
        self.assertEqual(closest("travel", ["gravel", "gambit", "gated"], algorithm="jaro"), "gravel")
        self.assertEqual(closest("travel", ["gravel", "gambit", "gated"], algorithm="jarowinkler"), "gravel")
        self.assertEqual(closest("travel", ["gravel", "gambit", "guards"], algorithm="hamming"), "gravel")
        with self.assertRaises(ValueError):
            closest("travel", ["gravel", "gambit", "gated"], algorithm="unknown")
        with self.assertRaises(ValueError):
            closest("travel", ["gravel", "gambit", "gated"], algorithm="hamming")
        with self.assertRaises(ValueError):
            closest("travel", [])    
    
    def test_n_closest(self):
        self.assertEqual(n_closest("hello", ["yello", "jello", "harps", "languid"], n=2), ["yello", "jello"])
        self.assertEqual(n_closest("hello", ["yello", "jello", "harps", "languid"], n=3), ["yello", "jello", "harps"])
        self.assertEqual(n_closest("hello", ["yello", "jello", "harps", "languid"], n=3, algorithm="jaro"), ["yello", "jello", "harps"])
        self.assertEqual(n_closest("hello", ["yello", "jello", "harps", "languid"], n=3, algorithm="jarowinkler"), ["yello", "jello", "harps"])
        with self.assertRaises(ValueError):
            n_closest("travel", ["gravel", "gambit", "gated"], n=2, algorithm="unknown")
        with self.assertRaises(ValueError):
            n_closest("travel", ["gravel", "gambit", "gated"], n=2, algorithm="hamming")
        with self.assertRaises(ValueError):
            n_closest("travel", [], n=2)    
        with self.assertRaises(ValueError):
            n_closest("travel", ["train", "tracks", "towered"], n=0)

if __name__ == '__main__':
    unittest.main()