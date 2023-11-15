let roomListDiv = document.getElementById('room-list');
let messagesDiv = document.getElementById('messages');
let newMessageForm = document.getElementById('new-message');
let newRoomForm = document.getElementById('new-room');
let statusDiv = document.getElementById('status');
let loginModal = document.getElementById('login-modal');

let roomTemplate = document.getElementById('room');
let messageTemplate = document.getElementById('message');

let messageField = newMessageForm.querySelector("#message");
let usernameField = newMessageForm.querySelector("#username");
let roomNameField = newRoomForm.querySelector("#name");


var STATE = {
    room: "Lobby",
    rooms: {},
    connected: false,
    ready: false
}


// Generate a color from a "hash" of a string. Thanks, internet.
function hashColor(str) {
    let hash = 0;
    for (var i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
        hash = hash & hash;
    }

    return `hsl(${hash % 360}, 100%, 70%)`;
}

async function findRoomByName(name) {
    const URI = "/diesel/room/name/" + String(name);
    let response = await fetch(URI,
        {
            method: "GET"
        })
        .then((response) => response.json())
        .then((data) => {
            console.log(`Room (${name}) got response [${data.result}]`);
            return data;
        })

    return response;
}

async function createNewRoom(name) {
    var id = null;
    const response = await findRoomByName(name);
    if (!response.authorized) {
        const URI = "/diesel/room";
        const msg = { room_name: name };
        const creation = await fetch(URI,
            {
                method: "POST",
                body: JSON.stringify(msg)
            })
            .then((response) => response.json())
            .then((data) => { return data; })

        id = creation.id;
    } else {
        id = response.id;
    }
    console.log(`Room ${name} has id ${id}`);
    STATE.rooms[name] = id;
}


// Add a new room `name` and change to it. Returns `true` if the room didn't
// already exist and false otherwise.
async function addRoom(name) {
    if (STATE[name]) {
        // it's already been added
        changeRoom(name);
        return false;
    }

    createNewRoom(name);

    // add it to the room list on the sidebar
    let node = roomTemplate.content.cloneNode(true);
    let room = node.querySelector(".room");
    room.addEventListener("click", () => changeRoom(name));
    room.textContent = name;
    room.dataset.name = name;
    roomListDiv.appendChild(node);

    // write to the database
    // STATE.rooms.push(name); // REWRITE
    STATE[name] = [];
    changeRoom(name);
    return true;
}

// Change the current room to `name`, restoring its messages.
function changeRoom(name) {
    if (STATE.room == name) return;

    var newRoom = roomListDiv.querySelector(`.room[data-name='${name}']`);
    var oldRoom = roomListDiv.querySelector(`.room[data-name='${STATE.room}']`);
    if (!newRoom || !oldRoom) return;

    STATE.room = name;
    oldRoom.classList.remove("active");
    newRoom.classList.add("active");

    messagesDiv.querySelectorAll(".message").forEach((msg) => {
        messagesDiv.removeChild(msg)
    });

    STATE[name].forEach((data) => addMessage(name, data.username, data.message))
}

// Add `message` from `username` to `room`. If `push`, then actually store the
// message. If the current room is `room`, render the message.
function addMessage(room, username, message, push = false) {
    if (push) {
        STATE[room].push({ username, message })
    }

    if (STATE.room == room) {
        var node = messageTemplate.content.cloneNode(true);
        node.querySelector(".message .username").textContent = username;
        node.querySelector(".message .username").style.color = hashColor(username);
        node.querySelector(".message .text").textContent = message;
        messagesDiv.appendChild(node);
    }
}

function writeMessageToDb(room, user, message) {
    // write to the DB
    const msg = { user_id: user, room_id: room, body: message };
    fetch("/diesel", {
        method: "POST",
        body: JSON.stringify(msg)
    }).then((response) => {
        if (response.ok) messageField.value = "";
    });
}

// Subscribe to the event source at `uri` with exponential backoff reconnect.
function subscribe(uri) {
    var retryTime = 1;

    function connect(uri) {
        const events = new EventSource(uri);

        events.addEventListener("message", (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            const msg = JSON.parse(ev.data);
            if (!("message" in msg) || !("room" in msg) || !("username" in msg)) return;
            addMessage(msg.room, msg.username, msg.message, true);
        });

        events.addEventListener("open", () => {
            setConnectedStatus(true);
            console.log(`connected to event stream at ${uri}`);
            retryTime = 1;
        });

        events.addEventListener("error", () => {
            setConnectedStatus(false);
            events.close();

            let timeout = retryTime;
            retryTime = Math.min(64, retryTime * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => connect(uri), (() => timeout * 1000)());
        });
    }

    connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status) {
    STATE.connected = status;
    statusDiv.className = (status) ? "connected" : "reconnecting";
}

function loadPreviousMessages(previousMessages) {
    previousMessages
        .map((obj) => { return obj.room; })
        .filter((element, index, arr) => {
            return arr.indexOf(element) === index;
        }).forEach((room) => addRoom(room));

    changeRoom("Lobby");

    previousMessages.forEach((obj) => {
        let { room, username, message } = obj;
        addMessage(room, username, message, true);
    })
}

async function loadInitialMessages() {
    addRoom("Lobby");
    addRoom("Rocket");

    // populate initial messages
    changeRoom("Lobby");
    addMessage("Lobby", "Rocket", "Hey! Welcome to your new chat room!", true);
    addMessage("Rocket", "Rocket", "This is another room. Neat, huh?", true);
}

// Let's go! Initialize the world.
async function InitChatRooms() {
    // need to test
    const displayName = localStorage.getItem("displayName");

    // set default display name
    const displayNameBox = newMessageForm.querySelector('#username');
    displayNameBox.placeholder = displayName;

    const uri = "/diesel/all_messages";
    const previousMessages = await fetch(uri,
        {
            method: "GET"
        }).then((response) => {
            let result = response.json();
            console.log(result);
            return result;
        });

    console.log(previousMessages);
    await loadInitialMessages();
    loadPreviousMessages(previousMessages);

    // Set up the form handler.
    newMessageForm.addEventListener("submit", (e) => {
        e.preventDefault();

        const room = STATE.room;
        const roomId = STATE.rooms[room];
        const userId = parseInt(localStorage.getItem("userId"));
        const message = messageField.value;
        const username = usernameField.value || localStorage.getItem("displayName");
        if (!message) return;

        if (STATE.connected) {
            // Write to the DB
            writeMessageToDb(roomId, userId, message);

            fetch("/message", {
                method: "POST",
                body: new URLSearchParams({ room, username, message }),
            }).then((response) => {
                if (response.ok) messageField.value = "";
            });
        }
    })

    // Set up the new room handler.
    newRoomForm.addEventListener("submit", (e) => {
        e.preventDefault();

        const room = roomNameField.value;
        if (!room) return;

        // if the room already exists, do nothing
        if (STATE[room]) return;

        roomNameField.value = "";
        if (!addRoom(room)) return;


        addMessage(room, "Rocket", `Look, your own "${room}" room! Nice.`, true);
    })

    // Subscribe to server-sent events.
    subscribe("/events");
}

// run the authenticator
const auth = new Auth();

if (loginForm) {
    const loginFields = ["login-email", "login-password"];
    new Login(loginForm, loginFields);

    const createUserFields = ["create-email", "create-password", "confirm-password", "display-name"];
    new CreateUser(createAccountForm, createUserFields);
}