---------------------------------------------------------------------------------------------------
Basic CLI steps
---------------------------------------------------------------------------------------------------
1. mkdir <folder name>
2. cd <folder name>
3. git init
4. <editor> .gitignore # add **/target/ and **/Cargo.lock (and other files to ignore)
5. cargo new <package name> --vcs none # disables cargo auto git init
6. git add <file name> # first file to commit, usually the gitignore
7. git commit -m "Initial commit"
8. gh repo create <repo name> --public --source=. --remote=origin --push
