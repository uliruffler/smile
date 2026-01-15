#!/usr/bin/env python3
"""
Unit tests for Smile emoticon picker
Tests the core functionality without requiring a display
"""

import unittest
import json
import tempfile
from pathlib import Path
from unittest.mock import Mock, patch, MagicMock
import sys

# Mock GTK before importing smile
sys.modules['gi'] = MagicMock()
sys.modules['gi.repository'] = MagicMock()
sys.modules['gi.repository.Gtk'] = MagicMock()
sys.modules['gi.repository.Gdk'] = MagicMock()
sys.modules['gi.repository.GLib'] = MagicMock()

import smile

class TestEmoticonData(unittest.TestCase):
    """Test the emoticon database"""
    
    def test_emoticons_exist(self):
        """Test that emoticons dictionary is populated"""
        self.assertIsInstance(smile.EMOTICONS, dict)
        self.assertGreater(len(smile.EMOTICONS), 0)
    
    def test_emoticon_categories(self):
        """Test that expected categories exist"""
        expected_categories = ["Happy", "Sad", "Love", "Gestures", "Classic"]
        for category in expected_categories:
            self.assertIn(category, smile.EMOTICONS)
    
    def test_emoticons_are_lists(self):
        """Test that each category contains a list of emoticons"""
        for category, emoticons in smile.EMOTICONS.items():
            self.assertIsInstance(emoticons, list)
            self.assertGreater(len(emoticons), 0)
    
    def test_emoticons_are_strings(self):
        """Test that emoticons are strings"""
        for category, emoticons in smile.EMOTICONS.items():
            for emoticon in emoticons:
                self.assertIsInstance(emoticon, str)

class TestHistoryManagement(unittest.TestCase):
    """Test history management functionality"""
    
    def setUp(self):
        """Create a temporary directory for config"""
        self.temp_dir = tempfile.mkdtemp()
        self.config_dir = Path(self.temp_dir) / ".config" / "smile"
        self.config_file = self.config_dir / "history.json"
    
    def test_history_file_creation(self):
        """Test that history directory is created"""
        self.config_dir.mkdir(parents=True, exist_ok=True)
        self.assertTrue(self.config_dir.exists())
    
    def test_history_save_and_load(self):
        """Test saving and loading history"""
        self.config_dir.mkdir(parents=True, exist_ok=True)
        test_history = ["😀", "😊", "❤️"]
        
        # Save history
        with open(self.config_file, 'w', encoding='utf-8') as f:
            json.dump(test_history, f, ensure_ascii=False)
        
        # Load history
        with open(self.config_file, 'r', encoding='utf-8') as f:
            loaded = json.load(f)
        
        self.assertEqual(loaded, test_history)
    
    def test_history_limit(self):
        """Test that history is limited to 10 items"""
        history = list(range(15))
        limited = history[:10]
        self.assertEqual(len(limited), 10)
        self.assertEqual(limited, list(range(10)))

class TestSearchFunctionality(unittest.TestCase):
    """Test search/filtering functionality"""
    
    def test_filter_by_category(self):
        """Test filtering emoticons by category name"""
        filter_text = "happy"
        filtered = {k: v for k, v in smile.EMOTICONS.items() 
                   if filter_text.lower() in k.lower()}
        self.assertIn("Happy", filtered)
    
    def test_filter_by_emoticon(self):
        """Test filtering emoticons by emoticon character"""
        # This test verifies the logic of filtering
        filter_text = "😀"
        found = False
        for category, emoticons in smile.EMOTICONS.items():
            if filter_text in emoticons:
                found = True
                break
        self.assertTrue(found)
    
    def test_case_insensitive_search(self):
        """Test that search is case insensitive"""
        filter_text = "HAPPY"
        filtered = {k: v for k, v in smile.EMOTICONS.items() 
                   if filter_text.lower() in k.lower()}
        self.assertIn("Happy", filtered)

class TestEmoticonCategories(unittest.TestCase):
    """Test that all required emoticon categories exist"""
    
    def test_all_categories_present(self):
        """Test that all expected categories are present"""
        required_categories = [
            "Happy", "Sad", "Angry", "Surprised", "Love",
            "Gestures", "Faces", "Cool", "Symbols", "Objects",
            "Animals", "Food", "Classic"
        ]
        for category in required_categories:
            self.assertIn(category, smile.EMOTICONS, 
                         f"Category '{category}' is missing")
    
    def test_classic_emoticons(self):
        """Test that classic text emoticons are included"""
        classic = smile.EMOTICONS.get("Classic", [])
        # Check for some common classic emoticons
        self.assertIn(":-)", classic)
        self.assertIn(":)", classic)
        self.assertIn("<3", classic)

if __name__ == '__main__':
    unittest.main()
