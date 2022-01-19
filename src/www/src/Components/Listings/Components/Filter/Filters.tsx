
import Product from 'Components/Listings/cogs/product';
import Filter, { SortOption, SortOptions, PriceModifiers, Limits, PriceOrder } from 'Components/Listings/cogs/sort';

import CheckboxList from 'Components/Assets/List/Checkbox';
import { Item, mock$STORES, mock$BRANDS } from 'Components/Assets/List/list';

import { ReactComponent as Ascending } from 'assets/ascending.svg'
import React, { createRef, PropsWithChildren, useEffect, useMemo, useRef, useState } from 'react';
import EntryList from 'Components/Assets/List/Entry';

import './filters.css';
import useToggle from 'hooks/toggle';
import useObserve from 'hooks/observe';

type SetState<T> = React.Dispatch<React.SetStateAction<T>>;
interface Filters {
    Price: ({ prflt, setPrflt }: { prflt: PriceModifiers | undefined; setPrflt: SetState<PriceModifiers | undefined>; }) => JSX.Element;
    Store: ({ storelist,setStoreFlt }: { storelist?: Item[] | undefined,setStoreFlt: SetState<Set<string> | undefined> }) => JSX.Element;
    Brand: ({ brandlist }: { brandlist?: Item[] | undefined; }) => JSX.Element;
    Keyword: ({ setKeywordlist }: { setKeywordlist: SetState<Set<string>>; }) => JSX.Element;
}
const Filters: Filters = {
/** ### Price Filters
 * A collection of filters to sort by price.*/
    Price : ({prflt, setPrflt}: { prflt: PriceModifiers | undefined, setPrflt: SetState<PriceModifiers | undefined>}) => {
    // Order
    const [order,setOrder] = useState<PriceModifiers['order']>(prflt ? prflt.order : undefined);
    const lhRef = createRef<HTMLInputElement>();
    const hlRef = createRef<HTMLInputElement>();
    const onRadioChange = (e: React.ChangeEvent<HTMLInputElement>) => { 
        e.preventDefault();
        // var currValue = e.currentTarget.getAttribute("value");
        var currValue = e.target.value;
        (currValue && (currValue == "LH" || currValue == "HL"))
          ? setOrder(currValue)
          : setOrder(undefined);
    }
    const clearRadio = (e: React.MouseEvent<unknown>) => { 
        if (e.button == 0) {
            // console.log(lhRef,hlRef);
            if(lhRef.current) { lhRef.current.checked = false }
            if(hlRef.current) { hlRef.current.checked = false }
            setOrder(undefined);
        } 
    }
    
    // Range
    // const [range,setRange] = useState<Range>((prflt && !(typeof prflt == "string")) ? prflt.rng : { min: 0 , max: 1 });
    const [range,setRange] = useState<Limits>((prflt && prflt.rng) ? prflt.rng : {});
    const [rangeError, toggleRangeError] = useToggle(false);
    const [rngErrSymbol,setRNGErrSymbol] = useState<string>('-');
    const toggleRNGERRifT = (code?:number) => {if(rangeError)  {toggleRangeError(); rngErrorSymbolFromCode(code ? code : 0) }}
    const toggleRNGERRifF = () => {if(!rangeError) {toggleRangeError();}}
    const rngErrorSymbolFromCode = (code:number) => {
        switch(code) {
            case 1: setRNGErrSymbol('>'); break;
            case 2: setRNGErrSymbol('='); break;
           default: setRNGErrSymbol('-'); break;
        }
    }
    const preventPageScroll = (e: React.WheelEvent<HTMLInputElement>) => {
        e.stopPropagation();
        // e.currentTarget.blur();
        // setTimeout(() => { e.currentTarget.focus() }, 0); 
    }
                        //  setRange({min: num, max: num + 1})
    const rangeVerifier = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault();
        var currValue = e.currentTarget.getAttribute("data-value");
        console.log({...e});
        if (currValue) {
            // e.currentTarget.value = 
            console.log('e.target.value: ', e.currentTarget.value,
                        'e.target.value.replace():- ', e.currentTarget.value.replace(/[\d]/g, ''));
            // var num = parseInt(e.currentTarget.value, 10) ?? 0;
            const num = e.currentTarget.valueAsNumber;
            switch(currValue) {
                case "min": 
                    var setMin = (numb?:number) => setRange({min: numb, max: (range && range.max) ? range.max : undefined });
                    if (!isNaN(num)) {
                        if (range.max) { // If maximum exists
                            if (num < range.max) { // If typed number is less than range.max
                                setMin(num);
                                toggleRNGERRifT();
                            } else { console.log(`Minimum is greater than maximum(${num} > ${range.max})`); rngErrorSymbolFromCode((num == range.max) ? 2 : 1) ; toggleRNGERRifF(); }
                        } else { console.log("Empty `range.max`"); setMin(num); /* toggleRangeError(); */ }
                    } else { setMin(); toggleRNGERRifT(); }
                    break;
                case "max": 
                    var setMax = (numb?:number) => setRange({ min: (range && range.min) ? range.min : undefined , max: numb});
                    if (!isNaN(num)) {
                        if (range.min) {
                            if (num > range.min) {  // If typed number is greater than range.min
                                setMax(num);
                                toggleRNGERRifT();
                            } else {console.log(`Maximum is less than minimum(${num} < ${range.min})`); rngErrorSymbolFromCode((num == range.min) ? 2 : 1); toggleRNGERRifF();}
                        } else { console.log("Empty `range.min`"); setMax(num); /*  toggleRangeError(); */ }
                    } else { setMax(); toggleRNGERRifT(); }
                    break;
                default: console.log("Empty `data-value`"); 
            }
        }
    }

    useObserve(range,'range');
    useObserve(rangeError,'rangeError:');
    useObserve(rngErrSymbol,'rngErrorSymbol');

    useMemo(() => {
        if ( !rangeError ) {
            // && range && ((range.min && range.min >= 0) || (range.max && range.max > 1))) {
            // if (order) { setPrflt({order: order, rng: range}); }
            // else { setPrflt({rng: range})}
            // } else { setPrflt({order: order ? order : undefined}); }
            setPrflt({order: order, rng: range});
        }
    } ,[order,range]);

return (
    <div className="price-filter">
      <span className="price-range-filter">
        <label>Min<input type="number" min='0' data-value="min" placeholder='-' className={`price-range-box ${rangeError ? 'error' : ''}`} onWheel={preventPageScroll} onInput={rangeVerifier}/></label>
        {rngErrSymbol}
        <label>Max<input type="number" min='1' data-value="max" placeholder='-' className={`price-range-box ${rangeError ? 'error' : ''}`} onWheel={preventPageScroll} onInput={rangeVerifier}/></label>
      </span>
      <span className="price-order-filter" >
        <label onDoubleClick={clearRadio}>
            <input ref={lhRef} type="radio" id='pr-ord-radio' value="LH" name="price-order" className='price-order-radio' onInput={onRadioChange}/>
            <span><Ascending/>Low to High</span>
        </label>
        <label onDoubleClick={clearRadio}>
            <input ref={hlRef} type="radio" id='pr-ord-radio' value="HL" name="price-order" className='price-order-radio' onInput={onRadioChange}/>
            <span><Ascending/>High to Low</span>
        </label>
      </span>
    </div>
  )
    },

/** ### `Store Filter`
 * A stores list checkbox based filter.*/
    Store : ({storelist = mock$STORES,setStoreFlt}: { storelist?: Item[],setStoreFlt: SetState<Set<string> | undefined> } ) => {
    const [checkedSET, setCheckedSET] = useState<Set<string>>(new Set());
    useEffect(() => setStoreFlt(checkedSET) ,[checkedSET]);
    return (
        <div className='store-filter'>
            <CheckboxList list={storelist} selectHandler={setCheckedSET} />
            {/* <SortSET SET={checkedSET} setSET={setCheckedSET}/> */}
        </div>
    )
    },

/** ### `Brand Filter`
 * A brands checkbox list filter to sort by brands of products.*/
    Brand : ({ brandlist = mock$BRANDS }: { brandlist?: Item[] }) => {
    const [checkedSET, setCheckedSET] = useState<Set<string>>(new Set());
    return (
        <div className='brand-filter'>
            <CheckboxList list={brandlist} selectHandler={setCheckedSET} />
            {/* <SortSET SET={checkedSET} setSET={setCheckedSET} /> */}
        </div>
    )
    },

/** ###  `Keyword Filter`
 * A custom keyword list based filter that finds products based on the keywords in the list .*/
    Keyword : ({ setKeywordlist }: { setKeywordlist: SetState<Set<string>> }) => {
    return (
        <div className='brand-filter'>
            <EntryList entryHandler={setKeywordlist} />
            {/* <button className='sort-button' style={{visibility: 'hidden'}} /> */}
        </div>
    )
    },
}

export default Filters;