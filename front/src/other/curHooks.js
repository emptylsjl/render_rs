import {useCallback, useEffect, useRef, useState} from 'react';

const useStateAsyc = (initialState) => {
    const [state, setState] = useState(initialState);
    const resolverRef = useRef(null);

    useEffect(() => {
        if (resolverRef.current) {
            // eslint-disable-next-line
            resolverRef.current(state);
            resolverRef.current = null;
        }
        /**
         * Since a state update could be triggered with the exact same state again,
         * it's not enough to specify state as the only dependency of this useEffect.
         * That's why resolverRef.current is also a dependency, because it will guarantee,
         * that handleSetState was called in previous render
         */

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [resolverRef.current, state]);

    const handleSetState = useCallback((stateAction) => {
        setState(stateAction);
        return new Promise(resolve => {
            resolverRef.current = resolve;
        });
    }, [setState])

    return [state, handleSetState];
};

export {useStateAsyc}