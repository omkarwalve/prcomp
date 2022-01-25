import useObserve from "hooks/observe";
import useSET from "hooks/set";
import useToggle from "hooks/toggle";
import { listenerCount } from "process";
import React, { memo, useCallback, useEffect, useState } from "react";
import { Item, checkedSort } from './list';

import './list.css';

interface Move<Type> { 
    from: Type;
    to: Type;
}
interface CBXItemProps {
    item: Item;
    idx: number;
    setChecked: (item: string, op: "add" | "remove") => void;
    setReorder: React.Dispatch<React.SetStateAction<Move<number> | undefined>>; 
}
/** ### `CheckboxItem` 
 * An item for the CheckboxList. Can be `string` or `JSX.Element` */
const CheckboxItem = ({item,idx,setChecked,setReorder}: CBXItemProps ) => {
    const [checked, toggleChecked, setChk] = useToggle(item.checked ? item.checked : false);
    const onItemClick = (e : React.MouseEvent<HTMLLIElement>) => { e.stopPropagation(); toggleChecked(); }
    useEffect(()=>{ setChk(item.checked ? item.checked : false)}, [item.checked])
    // useObserve(checked,`checked for ${item.key}`);
    useEffect(() => {
        item.checked = checked;
        (checked)
        ? setChecked(item.key,"add")
        : setChecked(item.key,"remove")
    }, [checked]);

    // Events
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
        (index != indexOfDragged) && setReorder({from: indexOfDragged,to: index}) && console.log("Not sent");
        // setSETArray(array => reorderArrayMut(array,indexOfDragged,index));
        // toggleUpdate();
        // setSETArray(array => reorderArrayMut([...array],indexOfDragged,index));
    }
    return ( <li
               className={`list-item ${item.checked ? 'check' : '' }`} 
               key={item.key}
               onClick={onItemClick}
               draggable={`${item.checked ? 'true':'false'}`}
               onDragStart={(e) => onDragStart(e,idx)}
               onDragOver={onDragOver}
               onDrop={(e) => onDrop(e,idx)}
               > {item.value}
             </li>
           )
}

function reorderArray<Type>(array: Array<Type>,indexFrom: number, indexTo:number): Array<Type> {
    var targElem = array[indexFrom];
    var mIncrement = ( indexTo - indexFrom ) / Math.abs( indexTo - indexFrom );
    for(var elem = indexFrom; elem != indexTo; elem += mIncrement) {
        array[elem] = array[elem + mIncrement];
    }
    array[indexTo] = targElem;
    console.info('reorderArray:- ','reordered array: ', array);
    return array;
}

interface CBLProps {
    list: Item[];
    selectHandler: React.Dispatch<React.SetStateAction<Set<string>>>;
    placeholder?: string;
}
interface CheckDatalist<S,T> {
    checked: Set<S>;
    list: T[];
}
/** `CheckboxList` - A styled list with `checkboxes` as its list elements.
 *  Will return a `Set` of keys(`string`) for checked items.
 */
const CheckboxList = ({list,selectHandler,placeholder = "search for..."}: CBLProps) => {
    const [dataList, setDataList] = useState<typeof list>(list);
    const [checkedItems, updateCheckedItems, setChkItems] = useSET<string>();
    const updateCheckedCallback = useCallback(updateCheckedItems,[]);
    const [reOrder, setReorder] = useState<Move<number>>();
    const [filter, setFilterText] = useState<string>('');
    const onSearchTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault(); e.stopPropagation();
        setFilterText(e.currentTarget.value.trim() ?? '');
    }

    // useObserve(dataList,'dataList');
    // useEffect(() => { reOrder && setDataList(dlist => reorderArray([...dlist],reOrder.from,reOrder.to)); }, [reOrder]);
    useEffect(() => { reOrder && setChkItems(chkitems => new Set(reorderArray([...chkitems], reOrder.from, reOrder.to))); }, [reOrder]);

    useEffect(() => {
        if (filter)
        { setDataList(dlist => dlist.map(litem => { 
            (!litem.key.toLowerCase().includes(filter.toLowerCase()))
            ? litem.conceal = true
            : litem.conceal = false;
            return litem;
        })) }
        else { setDataList(dlist => dlist.map(litem => { litem.conceal = false; return litem })); }
    }, [filter]);

    useEffect(() => {
        setDataList(dlist => [...dlist].sort((a,b) => checkedSort(a,b,Array.from(checkedItems))))
        selectHandler(checkedItems);
    } , [checkedItems]);

    return (
        <div className="field-list">
            <input type="search" className="list-field" onChange={onSearchTextChange} placeholder={placeholder} />
            <ul className="list-items">
                {
                    dataList.map((litem,idx) => {
                        if (!litem.conceal){
                            return ( 
                               <CheckboxItem item={litem} idx={idx} setChecked={updateCheckedCallback} setReorder={setReorder}/> 
                            )
                        }
                    })
                }
            </ul>
        </div>
    )
}

export default memo(CheckboxList);