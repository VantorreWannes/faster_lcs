import random

def main(source: str, target: str) -> str:
    source = filter_non_occuring(source, target)
    target = filter_non_occuring(target, source)
    state = create_state(source, target)
    last = -1
    lcs = []
    for target_item, state_item in zip(target, state):
        if state_item > last:
            lcs.append(target_item)
            last = state_item
    return "".join(lcs)

def create_state(source: str, target: str) -> list[int]:
    n = len(source)
    m = len(target)

    # Initializing two lists of size m
    prev = [0] * (m + 1)
    cur = [0] * (m + 1)

    for idx1 in range(1, n + 1):
        for idx2 in range(1, m + 1):
            # If characters are matching
            if source[idx1 - 1] == target[idx2 - 1]:
                cur[idx2] = 1 + prev[idx2 - 1]
            else:
                # If characters are not matching
                cur[idx2] = max(cur[idx2 - 1], prev[idx2])

        prev = cur.copy()

    return cur

def filter_non_occuring(slice: str, other: str) -> str:
    return "".join([x for x in slice if x in other])

if __name__ == "__main__":
    SOURCE = "".join([str(x) for x in random.randbytes(1500)])
    TARGET = "".join([str(x) for x in random.randbytes(1500)])
    LCS = main(SOURCE, TARGET)
    print(LCS)
    print(len(LCS))
    
