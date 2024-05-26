# Readme for Developers

## 1. Project Overview

### Project Name

SUSTech Event

### Project Target

 Develop a comprehensive campus event service website to address these issues and make life easier for SUSTech students and faculty. The site will allow SUSTech students and faculty to browse information about various performances (e.g., musical concerts), lectures, competitions, and other events on campus and provide functions such as booking, purchasing tickets, and writing reviews. 

### Features

-   Responsive user interface
-   Visitor / CAS authentication for registration and login
-   Event list
-   Moment list

-   Comment
-   Chat room
-   Hilarious and cute stickers
-   Support images and videos
-   Security

### Technology Stack

- Vue.js
- Vue Router
- Vuex
- Element UI
- Axios

## 2. Start Now

### Install Dependencies

First, clone this project repository and install the dependencies.

```cmd
git clone https://github.com/sustech-cs304/team-project-24spring-team1.git
cd FrontEnd
npm install
```

### Launch Project

After installation, you can use `npm run serve` to launch the project.

## 3. Project Structure

```
.
├── CHANGELOG.md
├── CONTRIBUTING.md
├── ISSUE_TEMPLATE.md
├── LICENSE.md
├── README.md
├── backendChat.py
├── backendRegister.py
├── gitignore
├── intelij.webpack.js
├── mock
│   ├── index.js
│   ├── mock-server.js
│   ├── register.js
│   ├── table.js
│   ├── user.js
│   └── utils.js
├── package-lock.json
├── package.json
├── public
│   ├── apple-icon.png
│   ├── favicon.ico
│   ├── favicon.png
│   ├── img
│   ├── index.html
│   ├── manifest.json
│   └── robots.txt
├── src
│   ├── App.vue
│   ├── api
│   │   ├── register.js
│   │   └── user.js
│   ├── components
│   │   ├── BaseAlert.vue
│   │   ├── BaseButton.vue
│   │   ├── BaseCheckbox.vue
│   │   ├── BaseDropdown.vue
│   │   ├── BaseNav.vue
│   │   ├── BaseRadio.vue
│   │   ├── BaseTable.vue
│   │   ├── CloseButton.vue
│   │   ├── Modal.vue
│   │   ├── NavbarToggleButton.vue
│   │   └── index.js
│   ├── config.js
│   ├── directives
│   │   └── click-ouside.js
│   ├── i18n.js
│   ├── locales
│   │   ├── ar.json
│   │   └── en.json
│   ├── main.js
│   ├── pages
│   │   ├── Chat.vue
│   │   ├── Dashboard.vue
│   │   ├── Icons.vue
│   │   ├── Maps.vue
│   │   ├── MyEvent.vue
│   │   ├── MyMoment.vue
│   │   ├── NotFoundPage.vue
│   │   ├── Notifications.vue
│   │   ├── Profile.vue
│   │   ├── SettingProfile.vue
│   │   ├── TableList.vue
│   │   ├── Typography.vue
│   │   ├── adminPublish.vue
│   │   ├── login.vue
│   │   └── register.vue
│   ├── plugins
│   │   ├── RTLPlugin.js
│   │   ├── blackDashboard.js
│   │   ├── globalComponents.js
│   │   └── globalDirectives.js
│   ├── registerServiceWorker.js
│   ├── router
│   │   ├── index.js
│   │   ├── routes.js
│   │   └── starterRouter.js
│   └── settings.js
└── vue.config.js

```



## 4. Backend API

The backend provides APIs for managing events, user authentication, and other functionalities required by the SUSTech Event application. For detailed information on the available APIs, refer to the [API documentation](https://gist.github.com/YanWQ-monad/df20490a9097472f30c62c39c7b669af).

## 5. Frontend API

The chat page in the frontend provides APIs to enhance the performance. The details are shown in the [Chatpage API](https://github.com/advanced-chat/vue-advanced-chat/blob/main/README.md).

## 5. Contributing

We welcome contributions. Please follow these steps:

1. Fork the repository and clone it locally:

   ```
   git clone https://github.com/your-username/vue-black-dashboard.git
   cd vue-black-dashboard
   ```

2. Create a new branch:

   ```
   git checkout -b feature/your-feature
   ```

3. Commit your changes:

   ```
   git commit -am 'Add new feature'
   ```

4. Push

