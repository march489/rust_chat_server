const createAccountForm = document.getElementById('createAccountForm');

class CreateUser {
    constructor(form, fields) {
        this.form = form;
        this.fields = fields;
        this.validateOnCreate();
    }

    async createNewUser() {
        let result = false;
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
        // if (response.authorized) {
        //     console.log("new user CREATED");
        //     localStorage.setItem("userId", response.id);
        //     result = true;
        // }
        // return result;
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
                    if (response.authorized) {
                        // TODO: FILL THIS IN TOMORROW 
                        // BASED ON THE PARALLEL .THEN() CLAUSE
                        // in validateOnSubmit()
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