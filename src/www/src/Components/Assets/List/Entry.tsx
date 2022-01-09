import useSET from 'hooks/set';
import useToggle from 'hooks/toggle';
import React, { useEffect, useState } from 'react';
import { Item } from './list';

import './list.css';

interface EIProps {
     item: string,
     setChecked: (item: string, op: "add" | "remove") => void,
     remover: (item: string, op: "add" | "remove") => void 
}
const EntryItem = ({item,setChecked,remover}: EIProps) => {
    const [checked, toggleChecked] = useToggle(false);
    const onItemClick = () => toggleChecked();
    const onRemoveClick = () => {remover(item,"remove")};
    useEffect(() => {
        (checked)
        ? setChecked(item,"add")
        : setChecked(item,"remove")
    }, [checked]);
    return ( 
        <span className={`list-item ${checked ? 'check' : ''}`} onClick={onItemClick}>
            {item}
            <button className='entry-remove-button' onClick={onRemoveClick} />
        </span>
    )
}

interface ELProps { entryHandler: React.Dispatch<React.SetStateAction<Set<string>>> }
/** ## `EntryList`
 * A list creator with checkbox capabilities.  */
const EntryList = ({entryHandler}: ELProps) => {
    const [entryList,setEntryList] = useSET<string>();
    const [checkedItems, setCheckedItems] = useSET<string>();
    const onKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key == "Enter") {
            e.preventDefault(); e.stopPropagation();
            setEntryList(e.currentTarget.value.trim(), "add");
            e.currentTarget.value = '';
        }
    }

    useEffect(() => { entryHandler(checkedItems); } , [checkedItems]);

    return (
        <div className='field-list'>
            <input type="text" className="list-field" onKeyDown={onKeyDown} />
            <div className='list-items'>
                {
                    Array.from(entryList).map(entry => {
                        return (
                            <EntryItem item={entry} setChecked={setCheckedItems} remover={setEntryList} />
                        )
                    })
                }
            </div>
        </div>
    )
}

export default EntryList;