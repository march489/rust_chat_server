const logoutText = document.querySelector(".logout-text")

class Auth {
    constructor() {
        const auth = localStorage.getItem("auth");
        this.validateAuth(auth);
    }

    validateAuth(auth) {
        logoutText.addEventListener('click', (e) => {
            this.logout();
        });

        console.log("validating auth");
        if (auth != 1) {
            console.log("no auth found");
            $('.overlay').show();
        } else {
            // TODO: include pulls from localStorage of userId
            // and pass that to InitGameRooms()
            console.log("validated");
            InitChatRooms();
        }
    }

    logout() {
        localStorage.clear();
        window.location.replace("/");
    }
}