import Store from 'Components/Assets/Stores/Stores';
import Icon from 'Components/Assets/Stores/Stores';

/** ### `Item`
 *  A list item format for passing into different lists */
export interface Item {
    readonly key: string;
    readonly value: JSX.Element | string;
    conceal: boolean;
    checked?: boolean;
}
export function checkedSort(itemA: Item, itemB: Item,checklist: Item['key'][]): number {
    var order: number = 0;
    switch(true) {
        case (itemA.checked && itemB.checked) :
            order = checklist.indexOf(itemA.key) - checklist.indexOf(itemB.key);
            // # For default alphabetic dictionary sort :
                // order = itemA.key.toLowerCase().localeCompare(itemB.key.toLowerCase(),'en'); 
            break;
        case (itemA.checked):  
            order = -1;
            break;
        case (itemB.checked) : 
            order = 1;
            break;
        default:
            order = itemA.key.toLowerCase().localeCompare(itemB.key.toLowerCase(),'en');
            break;
    }
    return order;
}

export const blankItem: Item = {key: '', value: '',conceal: true }

/** ### Mock Stores List
 * A dummy store list for testing purposes. */
export const mock$STORES: Item[] = [
    Store.Item("amazon"),
    Store.Item("flipkart"),
    Store.Item("urbanladder"),
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