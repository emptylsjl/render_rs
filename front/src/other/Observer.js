function Observer() {
    let listeners = {};
    let states = {};

    function register(stateID, listener, value=null) {
        if (stateID in states) {
            states[stateID].listener.push(listener);
            listener[listener].push(stateID);
        } else {
            states[stateID] = {listener: [listener]};
            listeners[listener] = [stateID];
        }
        if (value != null) {
            states[stateID].value = value;
            states[stateID].listener.forEach(x => x(value));
        }
    }

    function drop(stateID=null, listener=null) {
        if (stateID) {
            states[stateID].listener.forEach((x) => {
                delete listeners[x];
            })
            delete states[stateID];
        } else if (listener) {
            listeners[listener].forEach((x) => {
                states[x].pop(listener)
            })
            delete listeners[listener]
        }
    }

    function set(stateID, value) {
        states[stateID].value = value
        // states[stateID].listeners.forEach((l) => l(value));
    }

    return {register, drop, set};
}

export default Observer
// const userStore = Observable({ name: "user", count: 0 });
//
// const useUser = () => {
//     const [user, setUser] = React.useState(userStore.get());
//
//     React.useEffect(() => {
//         return userStore.subscribe(setUser);
//     }, []);
//
//     const actions = React.useMemo(() => {
//         return {
//             setName: (name) => userStore.set({ ...user, name }),
//             incrementCount: () => userStore.set({ ...user, count: user.count + 1 }),
//             decrementCount: () => userStore.set({ ...user, count: user.count - 1 }),
//         }
//     }, [user])
//
//     return {
//         state: user,
//         actions
//     }
// }
