import STORE$icon from 'Components/Assets/Stores/Stores';

/** `CBLItem` - A checkbox list item format for passing into CBL */
export interface Item {
    key: string;
    value: JSX.Element | string;
    conceal: boolean;
}

/** ### Mock Stores List
 * A dummy store list for testing purposes. */
export const mock$STORES: Item[] = [
    { key: "amazon"     , value: <STORE$icon store='amazon'      vectored={false} />, conceal: false },
    { key: "flipkart"   , value: <STORE$icon store='flipkart'    vectored={false} />, conceal: false },
    { key: "urbanladder", value: <STORE$icon store='urbanladder' vectored={false} />, conceal: false },
];
/** ### Mock Brands List
 * A dummy brands list for testing purposes. */
export const mock$BRANDS: Item[] = [
    { key: "lg"       , value: "LG"       , conceal: false },
    { key: "godrej"   , value: "Godrej"   , conceal: false },
    { key: "whirlpool", value: "Whirlpool", conceal: false },
    { key: "onida"    , value: "Onida"    , conceal: false },
    { key: "sony"     , value: "Sony"     , conceal: false },
    { key: "videocon" , value: "Videocon" , conceal: false },
    { key: "xiaomi"   , value: "Xiaomi"   , conceal: false },
];