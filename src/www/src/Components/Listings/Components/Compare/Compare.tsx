//   __  __      __     __  ___ 
//  /  `/  \|\/||__)/\ |__)|__  
//  \__,\__/|  ||  /~~\|  \|___ 
//                              

// Library Imports
import React, { useState, useEffect } from "react";
// Hooks
import useToggle from 'hooks/toggle';

// File Imports
import { ReactComponent as Scale } from './assets/compare.svg';
import { ReactComponent as Close } from 'assets/close.svg';
import { ReactComponent as ThinScale } from './assets/scale.svg';
import Product from 'Components/Listings/cogs/product';
import Badge from 'Components/Assets/Badge/Badge';
import Icon from 'Components/Assets/Stores/Stores';
import Veil from 'Components/Assets/Veil/Veil';

// CSS
import './compare.css';
import Store from "Components/Assets/Stores/Stores";

export interface cmpActions {
    do: 'add' | 'delete';
    pid: string;
}
function cmpReducer(state: Set<string|unknown> , action: cmpActions): Set<string|unknown> {
    switch (action.do) {
        case 'add': 
            return new Set([...state,action.pid]);
        case 'delete': 
            return new Set([...state].filter(item => item!= action.pid));
        default:
            return state;
    }
}

/** `Add to Compare Button` */
const Checkbox = ({pid,setCompare}: {pid: string, setCompare: React.Dispatch<cmpActions>}) => {
    const [isCmpActive, setCmpActive] = useState<boolean>(false);
    const onCbtnClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        e.stopPropagation();
        setCmpActive(!isCmpActive);
    };
    useEffect(() => {
        (isCmpActive) 
                ? setCompare({do: 'add'   , pid: pid})
                : setCompare({do: 'delete', pid: pid})
            //setCompare(c => new Set([...c, pid]));
        //}
        //else { 
            //setCompare(c => new Set([...c].filter(item => item!= pid)));
        //}
    },[isCmpActive]);
    return (
            <button className={`compare-button ${isCmpActive ? 'check': ''}`} onClick={onCbtnClick}>
              <Scale />
            </button>
    )
};

/** `Product Comparison Table` */
const Table = ({cProducts}:{cProducts: Product[]}) => {
  const [specList, setSpecList ] = useState<object[]>([]);
  const [sKeys, setSKeys ] = useState<Set<string>>(new Set());

    const filterSpecs = (pList: Product[]): [Set<string>,object[]] => {
      let _set: Set<string> = new Set(); // Set of Specification Keys
        let _arr: object[] = []; // Array of all the specs objects
        pList.forEach(lItem => {
            let spec = (lItem.specs) ? lItem.specs : {};
            _arr.push(spec);
            Object.keys(spec).forEach(item => _set.add(item.trim())); 
        });
        return [_set, _arr];
    };

    useEffect(() => {
        let [uniqKeys, slist] = filterSpecs(cProducts);
        setSKeys(new Set(Array.from(uniqKeys).sort()));
        setSpecList(slist);
        //console.log(sKeys,specList);
    },[cProducts]);

    return (
            <table className="compare-table">
                <tr className="compare-row-image">
                    <th /> {/* Top Left Blank Gap*/}
                    {
                        cProducts.map(product => {
                            return (
                                <td>
                                    <img className="compare-pimage" src={product.img} alt="error"/>
                                    <a href={product.url} target="_blank" rel="noreferer">
                                        <Store.SVG store={product.store} />
                                    </a>
                                </td>
                        )})
                    }
                </tr>
                <tr className="row-pname">
                    <th className="compare-header">Product</th>
                    {
                        cProducts.map(product => { return( <td> {product.name} </td>)})
                    }
                </tr>
                {
                    Array.from(sKeys).sort().map(key => {
                        return(
                            <tr>
                                <th className="compare-header">{key}</th>
                                {
                                    specList.map(specObj => {
                                        let sVal = specObj[key as keyof typeof specObj];
                                        return(<td className="spec-cell">{(sVal != null) ? sVal : "-"}</td>)
                                    })
                                }
                            </tr>
                        )
                    })
                }
            </table>
    )
};


const Compare = ({products,compareSet}: {products: Product[], compareSet: Set<string|unknown>}) => {
    let [len, setLen] = useState(compareSet.size);
    //let [len, setLen] = useState(compareSet.length);
    const filterPDXBySpecs = (c1: Product[],c2: Set<string|unknown>) => { return c1.filter(elem => c2.has(elem.id))}
    const [prodData, setProdData] = useState<Product[]>([]); 

    // const [cloak,toggleCloak] = useToggle(false);
    const [isCWinActive, toggleCWin] = useToggle(false);
    const onCButtonClick = () => {  toggleCWin();  }

    //useEffect(() => { setCloak(isCWinActive); }, [isCWinActive]);
        //setProdData(products.filter(product => compareSet.has(product.id)));
        //console.log(compareSet,prodData);
    useEffect(()=> {
        setLen(compareSet.size);
        setProdData(filterPDXBySpecs(products,compareSet));
    },[compareSet]);

                //<h6>{len} items to compare..</h6>
    return (
        <>
            <Veil cloak={isCWinActive} toggler={toggleCWin} />
            <button onClick={onCButtonClick} className={`compare-trigger ${len > 1 ? 'active': ''}`}>
                <ThinScale/>
                <Badge data={len}/>
            </button>
            <div className={`compare-window ${isCWinActive ? 'active': ''}`}>
                <button className="compare-close" onClick={onCButtonClick}><Close /></button>
                <Table cProducts={prodData} />
            </div>
        </>
    )
};

export { Checkbox, Table, cmpReducer };
export default Compare;
