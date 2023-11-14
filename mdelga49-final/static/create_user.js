const createAccountForm = document.getElementById('createAccountForm');
const createAccountButton = document.getElementById('create-account-button');

class CreateUser {
    constructor(form, fields) {
        this.form = form;
        this.fields = fields;
        this.validateOnCreate();
    }

    async createNewUser() {
        const newUser = { email: this.enteredEmail, password: this.enteredPassword };

        let data = await fetch("/auth",
            {
                method: "POST",
                body: JSON.stringify(newUser)
            }).then((data) => {
                return data;
            });

        const response = await data.json();
        return response;
    }

    isValidEmail(email) {
        return String(email).toLowerCase().match(emailRegex);
    }

    isValidPassword(pw) {
        return pw.length >= 8;
    }

    async validateOnCreate() {
        createAccountForm.addEventListener('submit', (e) => {
            e.preventDefault();
            let error = 0;

            this.fields.forEach((field) => {
                const input = createAccountForm.querySelector(`#${field}`);
                if (!this.validateCreationFields(input)) {
                    error++;
                }
            });

            if (error == 0) {
                // create new user
                this.createNewUser().then((response) => {
                    const errorMessageSpan = createAccountButton.parentElement.querySelector('.error-message');

                    if (response.authorized) {
                        errorMessageSpan.innerText = "";
                        const newUserModal = document.getElementById('new-user-modal');
                        newUserModal.classList.add('close-modal');
                        $('.overlay').hide();

                        // save data
                        localStorage.setItem("auth", 1);
                        localStorage.setItem("time", Date.now());
                        localStorage.setItem("userId", response.id);

                        // load data
                        InitChatRooms();
                    } else {
                        errorMessageSpan.innerText = response.reason;
                    }
                })
            }
        })
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

    validateCreationFields(field) {
        if (field.value.trim() === "") {
            this.setStatus(
                field,
                `${field.previousElementSibling.innerText} cannot be blank`,
                "error"
            );
            return false;
        } else {
            if (field.id == "create-password") {
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
            } else if (field.id == "create-email") {
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
            } else if (field.id == "confirm-password") {
                // must match previous password
                if (field.value !== this.enteredPassword) {
                    this.setStatus(
                        field,
                        "Passwords must match",
                        "error"
                    );
                    return false;
                }
                this.setStatus(field, null, "success");
                return true;
            } else {
                throw new Error("something is wrong here");
            }
        }
    }
}