enum SomeTraitImpl{
   typeA(Struct A) 
   typeB(Struct B)
}

把所有实现some trait 的struct 用enum 表示， 丢进函数体后还要做分发

match sometraitimpl{
        typeA(valuea:typeA) =>{},
        typeB(valueb:TypeB) =>{},
}

直接用trait obejct做参数 
(degree: Box<dyn BachelorsDegree>) 
不用分发，直接调用 trait 的 关联func
