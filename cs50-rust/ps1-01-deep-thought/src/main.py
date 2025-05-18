def main():
   great_answer_life = ["42", "forty two", "forty-two", "my love caron"]
   user_input = input("What is the Great Question of Life, the Universe and Everything?\n")
   if user_input.strip().lower() in great_answer_life:
       print("Yes")
   else:
       print("No")

main()
