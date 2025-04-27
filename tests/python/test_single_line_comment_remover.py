import pytest
from single_line_comment_remover import remove_single_comments

def test_remove_single_comments_basic():
    content = """let x = 1; // comment 1
    // comment 2
    let y = 2;
"""
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; \n    \n    let y = 2;\n"

def test_remove_single_comments_with_single_character_markers():
    content = """let x = 1; # comment 1
    # comment 2
    let y = 2;
"""
    result = remove_single_comments(content, ["#"])
    assert result == "let x = 1; \n    \n    let y = 2;\n"

def test_comment_at_end_of_line():
    content = """let x = 1; //
    let y = 2;
//"""
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; \n    let y = 2;\n"

def test_no_newline_after_comment():
    content = "let x = 1; // comment"  # no newline at end
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; "

def test_multiple_comment_same_line():
    content = """let x = 1; // comment 1 // comment 2
    let y = 2;////"""
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; \n    let y = 2;"

def test_do_not_remove_when_one_comment_character():
    content = "let x = 1; / comment 1"
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; / comment 1"

def test_two_comment_markers():
    content = """let x = 1; // comment 1
    let z = 3; ` comment 2
    let y = 2;"""
    result = remove_single_comments(content, ["//", "`"])
    assert result == "let x = 1; \n    let z = 3; \n    let y = 2;"

def test_two_comment_markers_same_line():
    content = "let x = 1; // comment 1 ` comment 2"
    result = remove_single_comments(content, ["//", "`"])
    assert result == "let x = 1; "

def test_empty_input():
    content = ""
    result = remove_single_comments(content, ["//"])
    assert result == ""

def test_empty_markers():
    content = "let x = 1; // comment"
    result = remove_single_comments(content, [])
    assert result == "let x = 1; // comment"

def test_comment_at_start():
    content = "// comment\nlet x = 1;"
    result = remove_single_comments(content, ["//"])
    assert result == "\nlet x = 1;"

def test_unicode_in_comment():
    content = "let x = 1; // 你好\nlet y = 2;"
    result = remove_single_comments(content, ["//"])
    assert result == "let x = 1; \nlet y = 2;"

def test_nested_markers():
    content = "let x = 1; /// comment\nlet y = 2;"
    result = remove_single_comments(content, ["//", "///"])
    assert result == "let x = 1; \nlet y = 2;"
