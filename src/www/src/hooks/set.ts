import { useState } from "react";

/** ## useSET<`Generic`>
 * A custom `useState` based hook for states with `Set` data type.
 * Includes special `updater()` function that can update(`add`|`remove`) the `state` of the `Set` 
 * without the need to write extra spread syntax everywhere.
 * Also returns the default `setState` from `useState` if need be.  */
function useSET<SomeType>(initValue ?: Set<SomeType>) : [ Set<SomeType>, (item: SomeType, op: 'add' | 'remove') => void, React.Dispatch<React.SetStateAction<Set<SomeType>>> ] {
    const [set,setSET] = useState<Set<SomeType>>((initValue) ? initValue : new Set());
    const updateSetState = (item: SomeType,op: 'add' | 'remove') => { 
        switch(op) {
            case 'add':
                setSET( set => new Set([...set,item]) );
                break;
            case 'remove':
                setSET( set => new Set([...set].filter(set_item => set_item!= item)) );
                break;
            default:
                console.log("Invalid Operation");
        }
    }

    return [set,updateSetState,setSET];
}

export default useSET;