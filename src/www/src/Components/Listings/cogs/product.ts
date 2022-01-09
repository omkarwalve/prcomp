//  .______   .______        ______    _______   __    __    ______ .___________.
//  |   _  \  |   _  \      /  __  \  |       \ |  |  |  |  /      ||           |
//  |  |_)  | |  |_)  |    |  |  |  | |  .--.  ||  |  |  | |  ,----'`---|  |----`
//  |   ___/  |      /     |  |  |  | |  |  |  ||  |  |  | |  |         |  |     
//  |  |      |  |\  \----.|  `--'  | |  '--'  ||  `--'  | |  `----.    |  |     
//  | _|      | _| `._____| \______/  |_______/  \______/   \______|    |__|     


/**
 * `Price` - A dataclass wrapper to store price as numeric and as raw currency.
 *  @property {string} display - Raw Currency String ( e.g '₹5,699')
 *  @property {string} amount  - Numeric value       ( e.g  '5699' ) */
class Price {
  readonly display: string;
  readonly amount: number;
  static CURRENCY = ['$','₹'];
  constructor(raw: string) {
    this.display  = raw;
    this.amount   = parseFloat(raw.match(/[0-9.-]+/g)?.join('') ?? '0.0');
    //console.log(this.display,this.amount);
  }
}

/**
 * `Product` - A product dataclass for
 *  storing all product information.
 * @property {string} name           - Product Name
 * @property {string} store          - Product Store
 * @property {string} return_replace - Product Returns Policy
 * @property {string} warranty       - Product Warranty Policy
 * @property {string} specs          - Product Image URL
 * @property {string} image          - Product Image URL
 * @property {string} price          - Product Price
 * @property {string} url            - Product URL
 * @property {string} id             - Product ID               */
class Product {
    readonly name: string;
    readonly store: string;
    readonly return_replace: string | null;
    readonly warranty: string | null;
    readonly specs: object | null;
    readonly price: Price;
    readonly img: string;
    readonly url: string;
    readonly id: string;
    cloaked: boolean;

    constructor(object: any) {
      this.name = object.name;
      this.store = object.store;
      this.return_replace = (object.warranty != "❔") ? object.return_replace : null;
      this.warranty = (object.warranty != "❔") ? object.warranty : null;
      this.specs = (object.specs != "❔") ? JSON.parse(object.specs) : null;
      this.price = new Price(object.price);
      this.img = object.img;
      this.url = object.url;
      this.id = object.id;
      this.cloaked = false;
    }

    static from(object: any) : Product[] {
      return object?.listings.map((productdata: any)  => {
        return new Product(productdata);
      });
    }
    shorten(): ShortProduct {
      return new ShortProduct(this);
    }
}


/**
 * `ShortProduct` - A mini product dataclass for
 *  places where only basal product information is required
 * @property {string} name  - Product name
 * @property {string} image - Product image
 * @property {string} price - Product price
 * @property {string} store - Product store
 * @property {string} url   - Product url
 */
class ShortProduct {
    readonly name: string;
    readonly image: string;
    readonly price: Price;
    readonly store: string;
    readonly url: string;

    constructor(product: Product) {
        this.name  = product.name;
        this.image = product.img;
        this.price = product.price;
        this.store = product.store;
        this.url   = product.url;
    }
}

export enum SortOption {
  price,
  store,
  brand,
  keyword
}
export type PriceModifiers = 'HL' | 'LH' | { order?: 'HL'| 'LH' , rng: Range };
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
export interface Range { min: number, max: number }
class Sort {
    private static LHSort = (pA: Product, pB: Product): number => { return (pA.price.amount - pB.price.amount); }
    private static HLSort = (pA: Product, pB: Product): number => { return (pB.price.amount - pA.price.amount); }
    private static RANGEFilter = (pArr: Product[], min: number, max: number, ord?: "LH" | "HL") => {
      pArr.forEach((pdt,idx)  => {
        if (pdt.price.amount < min || pdt.price.amount > max)  {
          pdt.cloaked = true; 
          pArr[idx] = pdt;
        } else { pdt.cloaked = false; } });
        if (ord) {
          if (ord == "LH") { pArr.sort(Sort.LHSort) }
          else if (ord == "HL") { pArr.sort(Sort.HLSort) }
          else {}
        }
    }
/** `sort()` - Used to sort a `Product[]` by passing a set of `SortOptions`.  */
    static sort(products: Product[], sortopt: SortOptions): Product[] {
      if (sortopt.priority.length) {
        sortopt.priority.slice().reverse().forEach(opt => {
          switch(opt) {
            //# Price Sorting
            case SortOption.price :
              console.log("Sorting by price..");
              if (sortopt.price == 'HL')      { products.sort(Sort.HLSort); }
              else if (sortopt.price == 'LH') { products.sort(Sort.LHSort); }
              else if (sortopt.price)         { Sort.RANGEFilter(products, sortopt.price.rng.min, sortopt.price.rng.max, sortopt.price.order); }
              else { console.error("Undefined SortOption: `Price`"); }
              break;
            //# Store Sorting
            case SortOption.store :
              console.log("Sorting by store..");
              break;
          }
          })
      } else { console.log("Nothing in priority specified. Skipping sort."); }

      return products;
    }
}

export { ShortProduct, Sort }
export default Product;
