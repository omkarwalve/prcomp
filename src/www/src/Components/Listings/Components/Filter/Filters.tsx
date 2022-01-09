
import Product, { Sort, SortOption, SortOptions, PriceModifiers, Range } from 'Components/Listings/cogs/product';

import CheckboxList from 'Components/Assets/List/Checkbox';
import { Item, mock$STORES, mock$BRANDS } from 'Components/Assets/List/list';

import { ReactComponent as Ascending } from 'assets/ascending.svg'
import React, { useEffect, useMemo, useState } from 'react';
import EntryList from 'Components/Assets/List/Entry';

import './filters.css';

/** ### Price Filters
 * A collection of filters to sort by price.*/
const Price = ({prflt, setPrflt}: { prflt: PriceModifiers | undefined, setPrflt: React.Dispatch<React.SetStateAction<PriceModifiers | undefined>>}) => {
    // Order
    const [order,setOrder] = useState<'LH' | 'HL' | undefined>(prflt && (typeof prflt == "string") ? prflt : (prflt && prflt.order) ? prflt.order : undefined);
    const onRadioChange = (e: React.ChangeEvent<HTMLInputElement>) => { 
        e.preventDefault();
        // var currValue = e.currentTarget.getAttribute("value");
        var currValue = e.target.value;
        (currValue && (currValue == "LH" || currValue == "HL"))
          ? setOrder(currValue)
          : setOrder(undefined);
    }
    // useEffect(() => {console.log(order)} , [order]);

    // Range
    const [range,setRange] = useState<Range>((prflt && !(typeof prflt == "string")) ? prflt.rng : { min: 0 , max: 1 });
    const preventPageScroll = (e: React.WheelEvent<HTMLInputElement>) => {
        e.stopPropagation();
        // e.currentTarget.blur();
        // setTimeout(() => { e.currentTarget.focus() }, 0); 
    }
    const onRangeInput = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault();
        var currValue = e.currentTarget.getAttribute("data-value");
        if (currValue) {
            var num = parseInt(e.currentTarget.value, 10) ?? 0;
            if (currValue == "min") {
                 (range.max) 
                  ? (num < range.max)
                    ? setRange({min: num, max: (range) ? range.max : 0 }) 
                    : console.log("Minimum is greater than maximum")
                  : console.log("Empty `range.max`")
            }
            else if (currValue == "max") {
                  (num > range.min)
                    ? setRange({ min: (range) ? range.min : 0, max: num}) 
                    : console.log("Maximum is less than minimum")
            }
            else { console.log("Empty `data-value`"); }
        }
    }
    // useEffect(() => {console.log(range)} , [range]);

    useMemo(() => {
        if (range.max > 1) {
            if (order) { setPrflt({order: order, rng: { min: range.min, max: range.max}}); }
            else { setPrflt({rng: {min: range.min,max: range.max}})}
        } else { setPrflt(order ? order : undefined); }
    } ,[order,range]);

   return (
    <div className="price-filter">
      <span className="price-range-filter">
        <label>Min:<input type="number" min='0' data-value="min" value={range.min} className="price-range-box" onWheel={preventPageScroll} onInput={onRangeInput}/></label>
        <label>Max:<input type="number" min='1' data-value="max" value={range.max} className="price-range-box" onWheel={preventPageScroll} onInput={onRangeInput}/></label>
      </span>
      <span className="price-order-filter">
        <label>
            <input type="radio" id='pr-ord-radio' value="LH" name="price-order" className='price-order-radio' onInput={onRadioChange}/>
            <span><Ascending/>Low to High</span>
        </label>
        <label>
            <input type="radio" id='pr-ord-radio' value="HL" name="price-order" className='price-order-radio' onInput={onRadioChange}/>
            <span><Ascending/>High to Low</span>
        </label>
      </span>
    </div>
  )
}

/** ### `Store Filter`
 * A stores list checkbox based filter.*/
const Store = ({storelist = mock$STORES}: { storelist?: Item[] } ) => {
    const [checkedSET, setCheckedSET] = useState<Set<string>>();
    return (
        <div className='store-filter'>
            <CheckboxList list={storelist} selectHandler={setCheckedSET} />
            <button className='sort-button'></button>
        </div>
    )
}

/** ### `Brand Filter`
 * A brands checkbox list filter to sort by brands of products.*/
const Brand = ({ brandlist = mock$BRANDS }: { brandlist?: Item[] }) => {
    const [checkedSET, setCheckedSET] = useState<Set<string>>();
    return (
        <div className='brand-filter'>
            <CheckboxList list={brandlist} selectHandler={setCheckedSET} />
            <button className='sort-button'></button>
        </div>
    )
}

/** ###  `Keyword Filter`
 * A custom keyword list based filter that finds products based on the keywords in the list .*/
const Keyword = ({ setKeywordlist }: { setKeywordlist: React.Dispatch<React.SetStateAction<Set<string>>> }) => {
    return (
        <div className='brand-filter'>
            <EntryList entryHandler={setKeywordlist} />
            <button className='sort-button'></button>
        </div>
    )
}


export { Price, Store, Brand, Keyword }