const loginButton = document.getElementById('login-button');
const loginForm = document.getElementById('loginForm');
const overlay = document.getElementById('overlay');
const emailRegex = /^(([^<>()[\]\.,;:\s@\"]+(\.[^<>()[\]\.,;:\s@\"]+)*)|(\".+\"))@(([^<>()[\]\.,;:\s@\"]+\.)+[^<>()[\]\.,;:\s@\"]{2,})$/i;

class Login {
    constructor(form, fields) {
        this.form = form;
        this.fields = fields;
        this.validateOnSubmit();
    }

    async getCredentialsAuthorized() {
        let result = false;
        const credentials = { email: this.enteredEmail, password: this.enteredPassword };

        let data = await fetch("/auth/shibboleth",
            {
                method: "POST",
                body: JSON.stringify(credentials)
            }).then((data) => {
                return data;
            });

        console.log(data);
        const response = await data.json();
        if (response.authorized) {
            console.log("We've been AUTHORIZED");
            result = true;
            localStorage.setItem("userId", response.id);
        }

        return result;
    }

    async validateOnSubmit() {
        let self = this;
        loginForm.addEventListener('submit', (e) => {
            e.preventDefault();
            let error = 0;

            // log to console so we can keep track
            self.fields.forEach((field) => {
                const input = loginForm.querySelector(`#${field}`);
                if (!self.validateLoginFields(input)) {
                    error++;
                }
            });

            if (error == 0) {
                // check the back end
                this.getCredentialsAuthorized().then((isAuthorized) => {
                    if (isAuthorized) {
                        loginModal.classList.add("close-modal");
                        $('.overlay').hide();
                        localStorage.setItem("auth", 1);
                        localStorage.setItem("time", Date.now());
                        InitGameRooms();
                    }
                })
            }
        })
    }

    isValidEmail(email) {
        return String(email).toLowerCase().match(emailRegex);
    }

    isValidPassword(pw) {
        return pw.length >= 8;
    }

    validateLoginFields(field) {
        if (field.value.trim() === "") {
            this.setStatus(
                field,
                `${field.previousElementSibling.innerText} cannot be blank`,
                "error"
            );
            return false;
        } else {
            if (field.type == "password") {
                if (!this.isValidPassword(field.value)) {
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} must have at least 8 characters`,
                        "error"
                    );
                    return false;
                } else {
                    this.setStatus(field, null, "success");
                    this.enteredPassword = field.value;
                    return true;
                }
            } else if (field.type == "email") {
                // this is the email field
                if (!this.isValidEmail(field.value)) {
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} must be a valid email address`,
                        "error"
                    );
                    return false;
                }
                this.setStatus(field, null, "success");
                this.enteredEmail = field.value;
                return true;
            } else {
                throw new Error("something is wrong here");
            }
        }
    }

    setStatus(field, msg, status) {
        const errorMessageSpan = field.parentElement.querySelector(".error-message");

        if (status == "success") {
            if (errorMessageSpan) {
                errorMessageSpan.innerText = "";
            }
            field.classList.remove(".input-error");
        }

        if (status == "error") {
            errorMessageSpan.innerText = msg;
            field.classList.add("input-error");
        }
    }
}



