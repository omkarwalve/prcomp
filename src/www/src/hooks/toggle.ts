
import {useState} from 'react';

/** ## useToggle
 * A custom `useState` based hook for `boolean` type `state`'s.
 * Returns a `toggler()` function that simply `NOT`'s(`!`) the `state`.
 * Also returns the default `setState` from `useState` if need be.  */
function useToggle(initValue: boolean) : [boolean, () => void, React.Dispatch<React.SetStateAction<boolean>>] {
    const [value,setValue] = useState<boolean>(initValue);
    const toggleValue = () => setValue(!value);
    return [value,toggleValue, setValue]
}

export default useToggle;
