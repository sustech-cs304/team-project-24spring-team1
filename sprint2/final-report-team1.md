# Final-report-team1

## 1. Metrics

#### FrontEnd

- Lines of Code 

  - Tool Used: `cloc`

  - Command: `cloc src/`

  - Result: 21502 lines of code

    | Language        | Files | Blank Lines | Comment Lines | Code Lines |
    | --------------- | ----- | ----------- | ------------- | ---------- |
    | SCSS            | 140   | 2312        | 1215          | 12136      |
    | Vuejs Component | 62    | 441         | 391           | 7563       |
    | JavaScript      | 30    | 99          | 82            | 1099       |
    | CSS             | 2     | 136         | 28            | 456        |
    | JSON            | 2     | 0           | 0             | 232        |
    | SVG             | 12    | 0           | 0             | 12         |
    | YAML            | 1     | 4           | 13            | 5          |
    | **Total**       | 249   | 2992        | 1729          | 21503      |

- Number of packages/modules 

  - Tool Used: Manual count via the `package.json` file
  - Result: 14 (dependencies) + 23 (devDependencies) = 37

- Number of source files

  - Tool Used: `find` command in the terminal
  - Command: `find src/ -type f -name "*.vue" -or -name "*.js" | wc -l`
  - Result: 93 source files

-  Number of dependencies

  - Tool Used: `npm list --depth=0`
  - Command: `npm list --depth=0`
  - Result: 36 dependencies

## 2. Documentation

Please refer to `readme.md` and `readme_for_developer.md`.

## 3. Tests

## 4. Build

- In the `vue.config.js` file, add `publicPath: './',` to `module.exports`
- Run `npm run build` in the `FrontEnd` folder.
- Find `dist` folder, open `index.html` in browser.

## 5. Deployment

In the `FrontEnd` folder:
- Create `Dockerfile` and `.dockerignore` files
- Run `sudo docker build -t sustech_event:dev .` to create the docker image named `sustech_event`
  Note that if it raises an error `ERROR [internal] load metadata for docker.io/library/node:alpine`, please change the `credsStore` value in `$HOME/.docker/config.json` from `desktop` to `osxkeychain`.
- Run `docker run -v ${PWD}:/app -v /app/node_modules -p 8089:8080 sustech_event:dev` to launch at port 8089
- Publish the image `sustech_event` to DockerHub