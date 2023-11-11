const logoutText = document.querySelector(".logout-text")

class Auth {
    constructor() {
        const auth = localStorage.getItem("auth");
        this.validateAuth(auth);
    }

    validateAuth(auth) {
        console.log("validating auth");
        if (auth != 1) {
            console.log("no auth found");
            $('.overlay').show();
        } else {
            console.log("validated");
            InitGameRooms();

            logoutText.addEventListener('click', (e) => {
                this.logout();
            });
        }
    }

    logout() {
        localStorage.clear();
        window.location.replace("/");
    }
}