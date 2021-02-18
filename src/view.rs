#[derive(Debug, Clone, Copy)]
pub struct View {
    org_x: u32,
    org_y: u32,
    w: u32,
    h: u32,
}
impl View {
    pub fn new((org_x, org_y): (u32, u32), (w, h): (u32, u32)) -> Self {
        let w = std::cmp::max(1, w);
        let w = std::cmp::min(u32::MAX - org_x, w - 1) + 1;
        let h = std::cmp::max(1, h);
        let h = std::cmp::min(u32::MAX - org_y, h - 1) + 1;
        Self { org_x, org_y, w, h }
    }
    pub fn default() -> Self {
        Self::new((0, 0), (u32::MAX, u32::MAX))
    }

    pub fn move_org(&self, (org_x, org_y): (u32, u32)) -> Self {
        Self::new((org_x, org_y), (self.w, self.h))
    }
    pub fn stretch(&self, (w, h): (u32, u32)) -> Self {
        Self::new((self.org_x, self.org_y), (w, h))
    }
}
impl From<View> for (u32, u32, u32, u32) {
    fn from(item: View) -> (u32, u32, u32, u32) {
        (item.org_x, item.org_y, item.w, item.h)
    }
}
impl From<&View> for (u32, u32, u32, u32) {
    fn from(item: &View) -> (u32, u32, u32, u32) {
        (item.org_x, item.org_y, item.w, item.h)
    }
}
