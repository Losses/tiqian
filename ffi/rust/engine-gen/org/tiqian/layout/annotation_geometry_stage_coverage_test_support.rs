use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::u_string;


#[derive(Clone, PartialEq)]
pub struct InkBoundsTextShaper {
    pub(crate) delegate: ExplainableStubTextShaper,
}

impl InkBoundsTextShaper {
    pub fn new(delegate: Option<ExplainableStubTextShaper>) -> Self {
        let delegate = delegate.unwrap_or_else(|| ExplainableStubTextShaper::new());
        Self {
            delegate: delegate,
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(1.0f64, 2.0f64, 9.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            let mut features: Vec<String> = vec![];
            for k in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                features.push((run.open_type_features[usize::try_from(k).unwrap_or(0)]).clone());
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), gs.to_vec(), run.advance, Some((features).clone())));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

impl ITextShaper for InkBoundsTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.AnnotationGeometryStageCoverageTestSupport.InkBoundsTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for j in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(j).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(1.0f64, 2.0f64, 9.0f64, 10.0f64)), g.halt_advance, g.halt_placement_x));
            }
            let mut features: Vec<String> = vec![];
            for k in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                features.push((run.open_type_features[usize::try_from(k).unwrap_or(0)]).clone());
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), gs.to_vec(), run.advance, Some((features).clone())));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

#[derive(Clone, PartialEq)]
pub struct MultiGlyphMinMaxShaper {
    pub(crate) delegate: ExplainableStubTextShaper,
}

impl MultiGlyphMinMaxShaper {
    pub fn new() -> Self {
        Self {
            delegate: ExplainableStubTextShaper::new(),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let g1 = Glyph::new(1u32, (input.range).clone(), 4.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(5.0f64, 5.0f64, 5.0f64, 5.0f64)), None, None);
            let g2 = Glyph::new(2u32, (input.range).clone(), 4.0f64, Some(4.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 10.0f64, 10.0f64)), None, None);
            let g3 = Glyph::new(3u32, (input.range).clone(), 4.0f64, Some(8.0f64), Some(0.0), None, Some(Rect::new(10.0f64, 10.0f64, 0.0f64, 0.0f64)), None, None);
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), vec![(g1).clone(), (g2).clone(), (g3).clone()].to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

impl ITextShaper for MultiGlyphMinMaxShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.AnnotationGeometryStageCoverageTestSupport.MultiGlyphMinMaxShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for i in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            let g1 = Glyph::new(1u32, (input.range).clone(), 4.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(5.0f64, 5.0f64, 5.0f64, 5.0f64)), None, None);
            let g2 = Glyph::new(2u32, (input.range).clone(), 4.0f64, Some(4.0f64), Some(0.0), None, Some(Rect::new(0.0f64, 0.0f64, 10.0f64, 10.0f64)), None, None);
            let g3 = Glyph::new(3u32, (input.range).clone(), 4.0f64, Some(8.0f64), Some(0.0), None, Some(Rect::new(10.0f64, 10.0f64, 0.0f64, 0.0f64)), None, None);
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_string().as_str(), vec![(g1).clone(), (g2).clone(), (g3).clone()].to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

#[derive(Clone, PartialEq)]
pub struct MultiGlyphBoundsShaper {
    pub call_count: u32,
}

impl MultiGlyphBoundsShaper {
    pub fn new() -> Self {
        Self {
            call_count: 0,
        }
    }

    pub fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.call_count += 1;
        let cluster = Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes())).as_str(), "test", 16.0f64,
Some((input.display_text).to_string()), Some(0.0), Some(0.0), Some(0.0));
        let mut glyphs: Vec<Glyph> = vec![];
        if u32::from_ne_bytes((i32::from_ne_bytes((self.call_count).to_ne_bytes()) % 2i32).to_ne_bytes()) == 0 {
            glyphs.push(Glyph::new(1u32, (input.range).clone(), 5.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(10.0f64, 10.0f64, 20.0f64, 20.0f64)), None, None));
            glyphs.push(Glyph::new(2u32, (input.range).clone(), 5.0f64, Some(5.0f64), Some(0.0), None, Some(Rect::new(5.0f64, 5.0f64, 25.0f64, 25.0f64)), None, None));
            glyphs.push(Glyph::new(3u32, (input.range).clone(), 6.0f64, Some(10.0f64), Some(0.0), None, Some(Rect::new(15.0f64, 15.0f64, 15.0f64, 15.0f64)), None, None));
        }
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(GlyphRun::new((input.range).clone(), "test", glyphs.to_vec(), 16.0f64, Some(vec![]))).clone()].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for MultiGlyphBoundsShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.AnnotationGeometryStageCoverageTestSupport.MultiGlyphBoundsShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.call_count += 1;
        let cluster = Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes())).as_str(), "test", 16.0f64,
Some((input.display_text).to_string()), Some(0.0), Some(0.0), Some(0.0));
        let mut glyphs: Vec<Glyph> = vec![];
        if u32::from_ne_bytes((i32::from_ne_bytes((self.call_count).to_ne_bytes()) % 2i32).to_ne_bytes()) == 0 {
            glyphs.push(Glyph::new(1u32, (input.range).clone(), 5.0f64, Some(0.0f64), Some(0.0), None, Some(Rect::new(10.0f64, 10.0f64, 20.0f64, 20.0f64)), None, None));
            glyphs.push(Glyph::new(2u32, (input.range).clone(), 5.0f64, Some(5.0f64), Some(0.0), None, Some(Rect::new(5.0f64, 5.0f64, 25.0f64, 25.0f64)), None, None));
            glyphs.push(Glyph::new(3u32, (input.range).clone(), 6.0f64, Some(10.0f64), Some(0.0), None, Some(Rect::new(15.0f64, 15.0f64, 15.0f64, 15.0f64)), None, None));
        }
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(GlyphRun::new((input.range).clone(), "test", glyphs.to_vec(), 16.0f64, Some(vec![]))).clone()].to_vec(), Some(vec![])));
    }
}
