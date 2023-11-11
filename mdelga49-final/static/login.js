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

    validateOnSubmit() {
        let self = this;
        loginForm.addEventListener('submit', (e) => {
            e.preventDefault();
            let error = 0;

            // log to console so we can keep track
            self.fields.forEach((field) => {
                const input = loginForm.querySelector(`#${field}`);
                if (!self.validateFields(input)) {
                    error++;
                }
            });

            if (error == 0) {
                console.log("success --> hit the backend");
                loginModal.classList.add("close-modal");
                $('.overlay').hide();
                localStorage.setItem("auth", 1);
                localStorage.setItem("time", Date.now());
                InitGameRooms();
            }
        })
    }

    validateFields(field) {
        if (field.value.trim() === "") {
            this.setStatus(
                field,
                `${field.previousElementSibling.innerText} cannot be blank`,
                "error"
            );
            return false;
        } else {
            if (field.type == "password") {
                if (field.value.length < 8) {
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} must have at least 8 characters`,
                        "error"
                    );
                    return false;
                } else {
                    this.setStatus(field, null, "success");
                    return true;
                }
            } else {
                // this is the email field
                if (!String(field.value).toLowerCase().match(emailRegex)) {
                    this.setStatus(
                        field,
                        `${field.previousElementSibling.innerText} must be a valid email address`,
                        "error"
                    );
                    return false;
                }
                this.setStatus(field, null, "success");
                return true;
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


if (loginForm) {
    const fields = ["login-email", "login-password"];
    const validator = new Login(loginForm, fields);
}
