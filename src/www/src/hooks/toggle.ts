
import {useState} from 'react';

function useToggle(initValue: boolean) : [boolean, () => void, React.Dispatch<React.SetStateAction<boolean>>] {
    const [value,setValue] = useState<boolean>(initValue);
    const toggleValue = () => setValue(!value);
    return [value,toggleValue, setValue]
}

export default useToggle;
