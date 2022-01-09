
import { useEffect, RefObject } from 'react';

type Event = MouseEvent | TouchEvent;

const useOnClickOutside = <T extends HTMLElement = HTMLElement>(
    ref: RefObject<T>,
    //toggler: React.Dispatch<React.SetStateAction<boolean>>,
    toggler: () => void,
    state: boolean
    ) => {
        const eventListener = (event: Event) => {
            const elem = ref?.current;
            // console.log(elem, event);
            if (elem && !elem.contains((event?.target as Node))) { 
                toggler();
                console.log("Toggle kar raha hu ab");
            }
            else { return; }
        }
        //useEffect(()=>{
          //(state)
          //? document.addEventListener('click', eventListener)
          //: document.removeEventListener('click', eventListener)
        //},[state]);
         useEffect(()=>{
            document.addEventListener('click', eventListener)
            return () => { document.removeEventListener('click', eventListener) }
        },[]);
}

export default useOnClickOutside;
