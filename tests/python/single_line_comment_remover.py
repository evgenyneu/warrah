def remove_single_comments(content: str, markers: list[str]) -> str:
    if not markers:
        return content

    result = []
    lines = content.splitlines()
    has_trailing_newline = content.endswith('\n')

    for i, line in enumerate(lines):
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

        # Add newline if not last line or if input ends with newline
        if i < len(lines) - 1 or has_trailing_newline:
            result.append('')

    # Join with newlines and ensure we don't have extra newlines at the end
    result_str = '\n'.join(result)
    if not has_trailing_newline and result_str.endswith('\n'):
        result_str = result_str[:-1]

    return result_str

# Performance test
if __name__ == "__main__":
    import time
    import tracemalloc

    print("== Single line comment removal performance")

    # Generate large content with comments (same as Rust test)
    content = ''
    for i in range(100000):
        content += f'let x{i} = {i}; // comment {i}\n'

    print(f"Input size: {len(content) / (1024 * 1024):.2f} MB")

    # Start memory tracking
    tracemalloc.start()

    # Measure time
    start_time = time.time()
    result = remove_single_comments(content, ["//", "<--"])
    duration = (time.time() - start_time) * 1000  # Convert to milliseconds

    # Get memory statistics
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    print(f"Output size: {len(result) / (1024 * 1024):.2f} MB")
    print(f"Processed in {duration:.6f}ms")
    print(f"Memory allocated (peak): {peak / (1024 * 1024):.2f} MB")
    print(f"Current memory usage: {current / (1024 * 1024):.2f} MB")
