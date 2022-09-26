use super::*;

pub fn raw<D: fmt::Display>(data: D) -> Raw<D> {
    Raw { data }
}

pub fn raw_escapable<D: fmt::Display>(data: D) -> RawEscapable<D> {
    RawEscapable { data }
}

pub fn from_iter<I: Iterator<Item = R>, R: RenderElem>(iter: I) -> Iter<I> {
    Iter { iter }
}

#[derive(Copy, Clone)]
pub struct Closure<I> {
    func: I,
}
pub fn from_closure<F: FnOnce(&mut WriteWrap) -> fmt::Result>(func: F) -> Closure<F> {
    Closure { func }
}

impl<I: FnOnce(&mut WriteWrap) -> fmt::Result> RenderElem for Closure<I> {
    type Tail = ();
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        (self.func)(w)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Iter<I> {
    iter: I,
}

impl<I: IntoIterator<Item = R>, R: RenderElem> RenderElem for Iter<I> {
    type Tail = ();
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        for i in self.iter {
            i.render_all(w)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Raw<D> {
    data: D,
}

impl<D: fmt::Display> RenderElem for Raw<D> {
    type Tail = ();
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        use std::fmt::Write;
        //TODO write one global function
        write!(crate::tools::escape_guard(w), " {}", self.data)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct RawEscapable<D> {
    data: D,
}

impl<D: fmt::Display> RenderElem for RawEscapable<D> {
    type Tail = ();
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        use std::fmt::Write;
        //TODO write one global function
        write!(w, " {}", self.data)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Single<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Single<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Single<D, AA> {
        Single {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Single<D, A> {
    type Tail = ();
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        use fmt::Write;
        let Single { tag, attr } = self;
        w.write_char('<')?;
        write!(crate::tools::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" />")?;
        Ok(())
    }
}

pub fn single<D: fmt::Display>(tag: D) -> Single<D, ()> {
    Single { tag: tag, attr: () }
}

pub fn elem<D: fmt::Display>(tag: D) -> Elem<D, ()> {
    Elem { tag, attr: () }
}

#[derive(Copy, Clone)]
pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display> RenderTail for ElemTail<D> {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result {
        use fmt::Write;
        w.write_str("</")?;
        write!(tools::escape_guard(&mut *w), "{}", &self.tag)?;
        w.write_char('>')
    }
}

#[derive(Copy, Clone)]
pub struct Elem<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Elem<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Elem<D, AA> {
        Elem {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Elem<D, A> {
    type Tail = ElemTail<D>;
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        let Elem { tag, attr } = self;

        use fmt::Write;
        w.write_char('<')?;
        write!(crate::tools::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" >")?;

        Ok(ElemTail { tag })
    }
}

#[derive(Copy, Clone)]
pub struct Path<I> {
    iter: I,
}

impl<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display> Attr for Path<I> {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result {
        use fmt::Write;

        w.write_str(" d=\"")?;

        for command in self.iter {
            command.write(tools::escape_guard(&mut *w))?;
        }
        w.write_str("\"")
    }
}

pub fn path<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display>(iter: I) -> Path<I> {
    Path { iter }
}

#[derive(Copy, Clone)]
pub struct Points<I> {
    iter: I,
}

pub fn points<I: IntoIterator<Item = (D, D)>, D: fmt::Display>(iter: I) -> Points<I> {
    Points { iter }
}

impl<I: IntoIterator<Item = (D, D)>, D: fmt::Display> Attr for Points<I> {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result {
        use fmt::Write;
        w.write_str(" points=\"")?;
        for (x, y) in self.iter {
            write!(tools::escape_guard(&mut *w), "{},{} ", x, y)?;
        }
        w.write_str("\"")
    }
}

///
/// Construct and Write a SVG path's data.
///
/// following: [w3 spec](https://www.w3.org/TR/SVG/paths.html#PathDataGeneralInformation)
///

#[derive(Copy, Clone)]
pub enum PathCommand<F> {
    /// move to
    M(F, F),
    /// relative move to
    M_(F, F),
    /// line to
    L(F, F),
    /// relative line to
    L_(F, F),
    /// horizontal to
    H(F),
    /// relative horizontal to
    H_(F),
    /// vertical to
    V(F),
    /// relative vertical to
    V_(F),
    /// curve to
    C(F, F, F, F, F, F),
    /// relative curve to
    C_(F, F, F, F, F, F),
    /// shorthand curve to
    S(F, F, F, F),
    /// relative shorthand curve to
    S_(F, F, F, F),
    /// quadratic bezier curve to
    Q(F, F, F, F),
    /// relative quadratic bezier curve to
    Q_(F, F, F, F),
    /// shorthand quadratic bezier curve to
    T(F, F),
    /// relative shorthand quadratic bezier curve to
    T_(F, F),
    /// elliptical arc
    A(F, F, F, F, F, F, F),
    /// relative elliptical arc
    A_(F, F, F, F, F, F, F),
    /// close path
    Z(),
}

impl<F> PathCommand<F> {
    #[inline(always)]
    fn write<T: fmt::Write>(&self, mut writer: T) -> fmt::Result
    where
        F: fmt::Display,
    {
        use PathCommand::*;
        match self {
            M(x, y) => {
                write!(writer, " M {} {}", x, y)
            }
            M_(x, y) => {
                write!(writer, " m {} {}", x, y)
            }
            L(x, y) => {
                write!(writer, " L {} {}", x, y)
            }
            L_(x, y) => {
                write!(writer, " l {} {}", x, y)
            }
            H(a) => {
                write!(writer, " H {}", a)
            }
            H_(a) => {
                write!(writer, " h {}", a)
            }
            V(a) => {
                write!(writer, " V {}", a)
            }
            V_(a) => {
                write!(writer, " v {}", a)
            }
            C(x1, y1, x2, y2, x, y) => {
                write!(writer, " C {} {}, {} {}, {} {}", x1, y1, x2, y2, x, y)
            }
            C_(dx1, dy1, dx2, dy2, dx, dy) => {
                write!(writer, " c {} {}, {} {}, {} {}", dx1, dy1, dx2, dy2, dx, dy)
            }
            S(x2, y2, x, y) => {
                write!(writer, " S {},{} {} {}", x2, y2, x, y)
            }
            S_(x2, y2, x, y) => {
                write!(writer, " s {},{} {} {}", x2, y2, x, y)
            }
            Q(x1, y1, x, y) => {
                write!(writer, " Q {} {}, {} {}", x1, y1, x, y)
            }
            Q_(dx1, dy1, dx, dy) => {
                write!(writer, " q {} {}, {} {}", dx1, dy1, dx, dy)
            }
            T(x, y) => {
                write!(writer, " T {} {}", x, y)
            }
            T_(x, y) => {
                write!(writer, " t {} {}", x, y)
            }
            A(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => {
                write!(
                    writer,
                    " A {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y
                )
            }
            A_(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => {
                write!(
                    writer,
                    " a {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy
                )
            }
            Z() => {
                write!(writer, " Z")
            }
        }
    }
}
