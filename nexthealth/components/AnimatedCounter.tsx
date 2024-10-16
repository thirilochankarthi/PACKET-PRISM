'use client';

import CountUp from 'react-countup';

const AnimatedCounter = ({amount}: {amount:number}) => {
  return (
    <div className="w-full">
        <CountUp 
        duration={1.5}
        decimal="."
        prefix="₹"
        decimals="2"
        end={amount} 
        />
    </div>
  )
}

export default AnimatedCounter