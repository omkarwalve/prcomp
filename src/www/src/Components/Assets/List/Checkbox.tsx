import useSET from "hooks/set";
import useToggle from "hooks/toggle";
import { listenerCount } from "process";
import React, { useEffect, useState } from "react";
import { Item } from './list';

import './list.css';

/** `CheckboxItem` - An item for the CheckboxList.
 *
 *   Can be `string` or `JSX.Element` */
const CheckboxItem = ({item,setChecked}: {item: Item, setChecked: (item: string, op: "add" | "remove") => void }) => {
    const [checked, toggleChecked] = useToggle(false);
    const onItemClick = () => toggleChecked();
    useEffect(() => {
        (checked)
        ? setChecked(item.key,"add")
        : setChecked(item.key,"remove")
    }, [checked]);
    return ( <span className={`list-item ${checked ? 'check' : '' }`} onClick={onItemClick}>{item.value}</span> )
}

interface CBLProps {
 list: Item[];
 selectHandler: React.Dispatch<React.SetStateAction<Set<string> | undefined>>;
 placeholder?: string;
}
/** `CheckboxList` - A styled list with `checkboxes` as its list elements.
 *  Will return a `Set` of keys(`string`) for checked items.
 */
const CheckboxList = ({list,selectHandler,placeholder = "search for..."}: CBLProps) => {
    const [dataList, setDataList] = useState<typeof list>(list);
    const [checkedItems, updateCheckedItems] = useSET<string>();
    const [filter,setFilterText] = useState<string>('');
    const onSearchTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault(); e.stopPropagation();
        setFilterText(e.currentTarget.value.trim() ?? '');
    }

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
        selectHandler(checkedItems);
    } , [checkedItems]);

    return (
        <div className="field-list">
            <input type="search" className="list-field" onChange={onSearchTextChange} placeholder={placeholder} />
            <div className="list-items">
                {
                    dataList.map(litem => {
                        if (!litem.conceal){
                            return( <CheckboxItem  item={litem} setChecked={updateCheckedItems} /> )
                        }
                    })
                }
            </div>
        </div>
    )
}

export default CheckboxList;