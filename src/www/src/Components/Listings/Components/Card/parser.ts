function periodParser(statement: string | null): string {
    var periods = [ "year" , "years",
                    "month", "months",
                    "day"  , "days"
                  ]
    return (statement) 
        ? statement
           .trim()
           .split(/\s/)
           .filter(elem => {
               if ( ! isNaN(parseInt(elem,10))
                   || periods.includes(elem.toLowerCase(),0)
                  ) { return true; }
               else { return false; }
           })
           .map(elem => {
               return elem;
           }).join(' ') 
        : ' - ';
}

export default periodParser;
