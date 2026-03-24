# This is week 1 of rust resolution
This is quite a basic app to fetch some data about github repositiories using its public api. 
You can fetch 4 different things about the repository at hand:
- Very basic data about the repo (repo)
- Its branches (branches)
- The languages used (languages)
- Number of open and closed issues and the titles of the most recent 10 open ones (issues)

## Usage
```
<path/to/executable> --repo <url> --option <repo|branches|languages|issues> 
