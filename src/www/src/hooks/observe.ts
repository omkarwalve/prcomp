import { useEffect } from "react"

/** ## useObserve<`Generic`>
 * A custom hook to log a `state` variable's changes into console.  */
export default function useObserve<SOME,>(state: SOME,stateName: string) {
    useEffect(() => {console.log(`${stateName}:`,state)},[state]);
}