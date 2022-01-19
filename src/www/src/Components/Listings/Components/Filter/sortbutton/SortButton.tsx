import useObserve from "hooks/observe";
import useToggle from "hooks/toggle";
import { PropsWithChildren, useEffect, useMemo, useState } from "react";
import { ReactComponent as SorterIcon } from 'assets/filter.svg'

// const SortSET = <SOME,>({set: SET,setSET}: {set: Set<SOME>,setSET: React.Dispatch<React.SetStateAction<Set<SOME> | undefined>>}) => {
interface SortSETProps<Type> {
    SET: Set<Type>,
    setSET: React.Dispatch<React.SetStateAction<Set<Type>>>
}
function reorderArrayMut<Type>(array: Array<Type>,indexFrom: number, indexTo:number): Array<Type> {
    var targElem = array[indexFrom];
    var mIncrement = ( indexTo - indexFrom ) / Math.abs( indexTo - indexFrom );
    for(var elem = indexFrom; elem != indexTo; elem += mIncrement) {
        array[elem] = array[elem + mIncrement];
    }
    array[indexTo] = targElem;
    console.info('reorderArrayMut:- ','reordered array: ', array);
    return array;
}
function reorderArray<Type>(array: Array<Type>,indexFrom: number, indexTo:number): Array<Type> {
    return array;
}
const SortSET = <SOME,>(props: PropsWithChildren<SortSETProps<SOME>>) => {
    const [showSorter,toggleSorter] = useToggle(false);
    const [update,toggleUpdate] = useToggle(false);
    const [SETArray,setSETArray] = useState<Array<SOME>>(Array.from(props.SET));
    const onSortClick = () => toggleSorter();
    const onDragStart = (e: React.DragEvent<HTMLLIElement>,index: number) => {
        // e.preventDefault();
        console.log("DRAGSTART:", index);
        e.dataTransfer.setData("index",index.toString());
    }
    const onDragOver = (e: React.DragEvent<HTMLLIElement>) => {
        e.preventDefault();
        e.stopPropagation();
        // console.log("DRAGOVER: ", index);
    }
    const onDrop = (e: React.DragEvent<HTMLLIElement>, index: number) => {
        e.preventDefault();
        const indexOfDragged = parseInt(e.dataTransfer.getData("index"), 10);
        console.log("DROPEVENT:- ", "from:", indexOfDragged, "to:", index );
        // setSETArray(array => reorderArrayMut(array,indexOfDragged,index));
        toggleUpdate();
        setSETArray(array => reorderArrayMut([...array],indexOfDragged,index));
    }
    useMemo(() => {setSETArray(Array.from(props.SET))}, [props.SET])
    useEffect(() => {props.setSET(new Set(SETArray))}, [update])

    useObserve(SETArray,"SETArray");
    return (
        <>
            <button className={`sorter-button ${props.SET.size > 1 ? 'active' : ''}`} onClick={onSortClick}>
                <SorterIcon />
            </button>
            <span className={`sorter-panel ${showSorter ? 'active' : ''}`}>
                <ul className='sorter-items'>
                    {
                        SETArray.map((SETitem,idx) => {
                            return ( 
                                <li 
                                    draggable="true"
                                    className='sorter-item'
                                    onDragOver={(e) => onDragOver(e)} 
                                    onDrop={(e) => onDrop(e,idx)}
                                    onDragStart={(e) => onDragStart(e,idx)} 
                                >
                                    {SETitem}
                                </li>
                            )
                        })
                    }
                </ul>
            </span>
        </>
    )
}

export default SortSET;