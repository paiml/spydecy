# Real-World Validation: List Operations
# This demonstrates multiple Spydecy patterns working together

def process_list(items):
    """Process a list with multiple operations - realistic use case."""
    # Check if list is empty
    if len(items) == 0:
        return None

    # Add new items
    items.append(42)
    items.append(99)

    # Check new length
    size = len(items)

    # Reverse the list
    items.reverse()

    # Get first item after reverse
    first = items.pop()

    return first
