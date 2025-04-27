# Python implementation of the single line comment remover
# to compare performance with Rust implementation
def remove_single_comments(content: str, markers: list[str]) -> str:
    if not markers:
        return content
    if not content:
        return ""

    result = []
    has_trailing_newline = content.endswith('\n')

    for line in content.splitlines():
        comment_start = None

        # Find earliest comment marker in the line
        for marker in markers:
            pos = line.find(marker)
            if pos != -1:
                comment_start = pos if comment_start is None else min(comment_start, pos)

        # Add line up to comment or whole line if no comment
        if comment_start is not None:
            result.append(line[:comment_start])
        else:
            result.append(line)

        result.append('\n')

    # Remove the last newline if it's not present in the original content
    if not has_trailing_newline and result:  # Check if result is not empty
        result.pop()

    return ''.join(result)

# Performance test
if __name__ == "__main__":
    import time

    print("== Single line comment removal performance")

    # Generate large content with comments (same as Rust test)
    content_parts = []
    for i in range(100000):
        content_parts.append(f'let x{i} = {i}; // comment {i}\n')
    content = ''.join(content_parts)

    print(f"Input size: {len(content) / (1024 * 1024):.2f} MB")

    # Measure time
    start_time = time.time()
    result = remove_single_comments(content, ["//", "<--"])
    duration = (time.time() - start_time) * 1000  # Convert to milliseconds

    print(f"Output size: {len(result) / (1024 * 1024):.2f} MB")
    print(f"Processed in {duration:.6f}ms")
