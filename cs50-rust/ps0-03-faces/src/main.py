def main():
    raw_input = input()
    print(replace_smiley(raw_input))

def replace_smiley(raw_input: str) -> str:
    return raw_input.replace(":)", "😊").replace(":(", "🙁")

if __name__ == "__main__":
    main()
