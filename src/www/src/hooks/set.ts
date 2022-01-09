import { useState } from "react";

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