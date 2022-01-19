import { parentPort } from "worker_threads";
import Product from "./product";

export enum SortOption {
  price,
  store,
  brand,
  keyword
}
export type PriceOrder = 'HL' | 'LH';
export type PriceModifiers = { order?: PriceOrder , rng?: Limits };
/** `Sorting Options` Object Type 
  * @property {'HL' | 'LH' | { order ?: 'HL' | 'LH', rng: {min: number , max: number} }} price   - Price sorting options 
  * @property {Set<string>}  store   - Store sorting options 
  * @property {Set<string>}  brand   - Brand sorting options 
  * @property {Set<string>}  keyword - Keyword sorting options 
  * @property {'price' | 'store' | 'brand' | 'keyword'[]} priority - Set priority of options ; */
export interface SortOptions {
    price    ?: PriceModifiers;
    store    ?: Set<string>;
    brand    ?: Set<string>;
    keyword  ?: Set<string>;
    priority : Array<SortOption>;
}
export interface Limits { min?: number, max?: number }
class Sort {
    /** ### Low -> High Sort `:: comparefn` */
    private static LHSort = (pA: Product, pB: Product): number => { return (pA.price.amount - pB.price.amount); }
    /** ### High -> Low Sort `:: comparefn` */
    private static HLSort = (pA: Product, pB: Product): number => { return (pB.price.amount - pA.price.amount); }
    /** ### Store Sort `:: comparefn`
     * Sort products based on order of stores specified by a `Array`. If not in `Array` the product will be cloaked.  */
    private static STORESort = (p0: Product, p1: Product, storeArray: string[]): number => {
        let order: number = 0;
        const idxes: [number,number] = [storeArray.indexOf(p0.store.toLowerCase()),storeArray.indexOf(p1.store.toLowerCase())];
        console.log('idxes:- ',idxes);
        // if (idxes[0] === -1) { p0.cloaked = true; } 
        // else if (idxes[1] === -1) { p1.cloaked = true; }
        // else if ((idxes[0] !== -1) && (idxes[1] !== -1)) { order = idxes[0] - idxes[1] }
        if ((idxes[0] !== -1) && (idxes[1] !== -1)) { order = idxes[0] - idxes[1] }
        return order;
    }
    /** ### Store Filter
     * Sort products based on order of stores specified by a `Array`. If not in `Array` the product will be cloaked.  */
    private static STOREFilter = (pArr: Product[], storeArray: string[]) => {
      pArr.forEach((pdt,idx) => {
        const idxST = storeArray.indexOf(pdt.store.toLowerCase());
        if (idxST === -1) { pdt.cloaked = true;  }
        else { pdt.cloaked = false; }
        pArr[idx] = pdt;
      });
    }
    /** ## sort() `:: mut,lazy,fast`
     *  Filters a `Product[]` one-by-one based on `SortOptions`. 
     *  Is faster than `oldsort()` since it splits up filtering work into `concealment` and `sorting` seperately 
     *  and is aware that `Product[].length` is always greater than No. of Filters.*/
    static sort(products: Product[], sortopt: SortOptions): Product[] {
        let storeArray = (sortopt.store) ? Array.from(sortopt.store) : [];
        // Sort Flags
        let [lh,hl]: boolean[] = Array(2).fill(false);
        products.forEach((pdt,idx) => {
            let [prCLK,stCLK,brCLK,kwCLK]: boolean[] = Array(4).fill(false);
            if (sortopt.priority.length) {
                sortopt.priority.slice().reverse().forEach(opt => {
                    switch(opt) {
                        case SortOption.price:
                            if (sortopt.price) {
                                if (sortopt.price.rng) {
                                    let max = sortopt.price.rng.max;
                                    let min = sortopt.price.rng.min;
                                    if (max && min) { prCLK = (pdt.price.amount < min || pdt.price.amount > max) ? true : false;
                                    } else if (min) { prCLK = (pdt.price.amount < min) ? true : false;
                                    } else if (max) { prCLK = (pdt.price.amount > max) ? true : false;
                                    } else { void(0) }
                                }
                                if (sortopt.price.order) {
                                    lh = (sortopt.price.order == "LH") ? true : false;
                                    hl = (sortopt.price.order == "HL") ? true : false;
                                }
                            }
                            break;
                        case SortOption.store:
                            if (sortopt.store) {
                                if(storeArray.length) {
                                    stCLK = (storeArray.indexOf(pdt.store.toLowerCase()) === -1) ? true : false;
                                }
                            }
                            break;
                        case SortOption.brand:
                            break;
                        case SortOption.keyword:
                            break;
                        // default:
                    }
                });
            }
            pdt.cloaked = (prCLK || stCLK || brCLK || kwCLK) ? true : false;
            // products[idx] = pdt;
        });

        sortopt.priority.slice().reverse().forEach(opt => {
            switch(opt) {
                case SortOption.price:
                    (lh) && products.sort(Sort.LHSort);
                    (hl) && products.sort(Sort.HLSort);
                    break;
                case SortOption.store:
                    (storeArray.length) && products.sort((a,b) => Sort.STORESort(a,b,storeArray));
                    break;
                case SortOption.brand:
                    break;
                case SortOption.keyword:
                    break;
            }
        });
        return products;
    }
    /** ## oldsort() `:: mut,aggressive,slowest`
     *  Filters a `Product[]` based on a set of `SortOptions`.
     *  Cloaks the items in `Products[]` based on the last `SortOption` in `priority`. 
     *  Is computationally very expensive since it is a `Filters` oriented sort and `Products[]` is iterated over multiple times for each `SortOption`. */
    static oldsort(products: Product[], sortopt: SortOptions): Product[] {
        /** ### Range Filter
         * To filter out products which do not fit in specified `Limits`.  */
        const RANGEFilter = (pArr: Product[], min?: number, max?: number, ord?: PriceModifiers['order']) => {
            // Range sort
            if (max && min) { 
                pArr.forEach((pdt,idx)  => {
                    if (pdt.price.amount < min || pdt.price.amount > max)  {
                    pdt.cloaked = true;
                    pArr[idx] = pdt;
                    } else { pdt.cloaked = false; pArr[idx] = pdt; } 
                });
            } else if (min) {
                pArr.forEach((pdt,idx)  => {
                    if (pdt.price.amount < min)  {
                        pdt.cloaked = true; 
                        pArr[idx] = pdt;
                    } else { pdt.cloaked = false; pArr[idx] = pdt;} 
                });
            } else if (max) {
                pArr.forEach((pdt,idx)  => {
                    if (pdt.price.amount > max)  {
                        pdt.cloaked = true; 
                        pArr[idx] = pdt;
                    } else { pdt.cloaked = false; pArr[idx] = pdt;} 
                });
            } else {
                //  console.info("Resetting sort since both undefined.")
                pArr.forEach((pdt,idx) => {pdt.cloaked = false; pArr[idx] = pdt;});
                }
        }
      if (sortopt.priority.length) {
        sortopt.priority.slice().reverse().forEach(opt => {
          switch(opt) {
            //# Price Sort
            case SortOption.price :
              console.log("Sorting by price..");
              if (sortopt.price) {
                  if (sortopt.price.rng) {
                     RANGEFilter(products, sortopt.price.rng.min, sortopt.price.rng.max);
                  }
                  if (sortopt.price.order) {
                         if (sortopt.price.order === 'HL') { products.sort(Sort.HLSort); }
                    else if (sortopt.price.order === 'LH') { products.sort(Sort.LHSort); }
                  }
              }
              else { console.error("Undefined SortOption: `Price`"); }
              break;
            //# Store Sort
            case SortOption.store :
              console.log("Sorting by store..");
              if (sortopt.store) {
                const storeArray = Array.from(sortopt.store);
                console.log("storeArray(checked):- ", storeArray);
                if(storeArray.length) { products.sort((a,b) => Sort.STORESort(a,b,storeArray)); }
                // if(storeArray.length) { Sort.STOREFilter(products,storeArray); }
              }
              break;
            //# Brand Sort
            case SortOption.brand :
              console.log("Sorting by brands...");
              break;
            //# Keyword Sort
            case SortOption.keyword :
              console.log("Sorting by keywords...");
              break;
          }
          })
      } else { console.log("Nothing in priority specified. Skipping sort."); }
      return products;
    }
}
export default Sort;