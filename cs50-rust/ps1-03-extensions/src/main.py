def main():
    file_name = input("File name: ")
    media_type(file_name)


def media_type(file_name: str):
   match file_name:
        case s if s.endswith(".gif"):
           print("image/gif")
        case s if s.endswith(".jpg"):
            print("image/jpg")
        case s if s.endswith(".jpeg"):
           print("image/jpeg")
        case s if s.endswith(".png"):
            print("image/png")
        case s if s.endswith(".pdf"):
            print("application/pdf")
        case s if s.endswith(".txt"):
            print("text/plain")
        case s if s.endswith(".zip"):
            print("application/zip")
        case _:
            print("application/octet-stream")


main()
