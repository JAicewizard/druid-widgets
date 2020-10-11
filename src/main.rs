use druid::kurbo::Size;
use druid::widget::prelude::*;
use druid::{
    AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    LocalizedString, PaintCtx, UpdateCtx, Widget, WidgetExt, WindowDesc,
};

pub fn main() {
    let window = WindowDesc::new(|| Dropdown::new()).title(
        LocalizedString::new("custom-widget-demo-window-title").with_placeholder("Fancy Colors"),
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(("select an element".into(), Testing::Unset))
        .expect("launch failed");
}
#[derive(Debug, Data, PartialEq, Clone)]
enum Testing {
    Unset,
    Hello,
    SixtyNine,
    Nice,
}

struct Dropdown<T> {
    button: druid::widget::ControllerHost<
        druid::widget::Label<(String, bool)>,
        druid::widget::Click<(String, bool)>,
    >,
    dropdown: druid::widget::Flex<(String, T)>,
    label_size: Size,
    dropdown_size: Size,
    is_open: bool,
}

impl Dropdown<Testing> {
    fn new() -> Self {
        let options: Vec<(String, Testing)> = vec![
            ("Hello".into(), Testing::Hello),
            ("69".into(), Testing::SixtyNine),
            ("nice".into(), Testing::Nice),
        ];

        let mut dropdown = druid::widget::Flex::column();
        let mut is_first = true;
        for (name, result) in options {
            if !is_first {
                dropdown = dropdown.with_child(
                    druid::widget::SizedBox::new(druid::widget::Painter::new(
                        |ctx, _data: &(String, Testing), env| {
                            let bounds = ctx.size().to_rect();
                            ctx.fill(bounds, &env.get(druid::theme::BORDER_DARK));
                        },
                    ))
                    .height(2.0),
                )
            }
            dropdown = dropdown.with_child(
                druid::widget::Align::left(druid::widget::Label::new(name.clone())).on_click(
                    move |_ctx: &mut EventCtx, data: &mut (String, Testing), _env: &Env| {
                        *data = (name.clone(), result.clone())
                    },
                ),
            );
            is_first = false;
        }

        Self {
            button: druid::widget::Label::dynamic(|data: &(String, bool), _env| data.0.clone())
                .on_click(
                    |_ctx: &mut EventCtx, data: &mut (String, bool), _env: &Env| data.1 = !data.1,
                ),
            dropdown,
            label_size: Size::ZERO,
            dropdown_size: Size::ZERO,
            is_open: false,
        }
    }
}

impl Widget<(String, Testing)> for Dropdown<Testing> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut (String, Testing),
        env: &Env,
    ) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.pos.x <= self.label_size.width
                    && mouse_event.pos.y <= self.label_size.height
                {
                    let mut button_data = (data.0.clone(), self.is_open);
                    self.button.event(ctx, event, &mut button_data, env);
                    self.is_open = button_data.1;
                    data.0 = button_data.0;
                    ctx.request_update();
                }

                let mut me = mouse_event.clone();
                me.pos -= druid::Vec2::new(0.0, self.label_size.height);
                if me.pos.x <= self.dropdown_size.width
                    && me.pos.x >= 0.0
                    && me.pos.y <= self.dropdown_size.height
                    && me.pos.y >= 0.0
                    && self.is_open
                {
                    self.dropdown
                        .event(ctx, &druid::Event::MouseDown(me), data, env);
                }
            }
            Event::MouseUp(mouse_event) => {
                if mouse_event.pos.x <= self.label_size.width
                    && mouse_event.pos.y <= self.label_size.height
                {
                    let mut button_data = (data.0.clone(), self.is_open);
                    self.button.event(ctx, event, &mut button_data, env);
                    self.is_open = button_data.1;
                    data.0 = button_data.0;
                    ctx.request_update();
                }

                let mut me = mouse_event.clone();
                me.pos -= druid::Vec2::new(0.0, self.label_size.height);
                if me.pos.x <= self.dropdown_size.width
                    && me.pos.x >= 0.0
                    && me.pos.y <= self.dropdown_size.height
                    && me.pos.y >= 0.0
                    && self.is_open
                {
                    self.dropdown
                        .event(ctx, &druid::Event::MouseUp(me), data, env);
                }
            }
            Event::MouseMove(mouse_event) => {
                let mut button_data = (data.0.clone(), self.is_open);
                self.button.event(ctx, event, &mut button_data, env);
                self.is_open = button_data.1;
                data.0 = button_data.0;

                if self.is_open {
                    let mut me = mouse_event.clone();
                    me.pos -= druid::Vec2::new(0.0, self.label_size.height);
                    self.dropdown
                        .event(ctx, &druid::Event::MouseMove(me), data, env);
                }
            }

            _ => {
                let mut button_data = (data.0.clone(), self.is_open);
                self.button.event(ctx, event, &mut button_data, env);
                self.is_open = button_data.1;
                data.0 = button_data.0;
                if self.is_open {
                    self.dropdown.event(ctx, event, data, env);
                }
            }
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &(String, Testing),
        env: &Env,
    ) {
        self.button
            .lifecycle(ctx, event, &mut (data.0.clone(), self.is_open), env);
        self.dropdown.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &(String, Testing),
        data: &(String, Testing),
        env: &Env,
    ) {
        self.button.update(
            ctx,
            &(old_data.0.clone(), self.is_open),
            &(data.0.clone(), self.is_open),
            env,
        );
        if old_data.1 != data.1 {
            self.is_open = false;
        }
        ctx.request_layout()
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &(String, Testing),
        env: &Env,
    ) -> Size {
        self.label_size =
            self.button
                .layout(ctx, &bc.loosen(), &mut (data.0.clone(), self.is_open), env);
        let dropdown_constraints = BoxConstraints::new(
            druid::Size::ZERO,
            druid::Size::new(self.label_size.width, std::f64::INFINITY),
        );

        self.dropdown_size = match self.is_open {
            true => self.dropdown.layout(ctx, &dropdown_constraints, data, env),
            false => Size::ZERO,
        };
        bc.constrain(self.label_size)
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &(String, Testing), env: &Env) {
        self.button
            .paint(ctx, &mut (data.0.clone(), self.is_open), env);
        if self.is_open {
            let layout_origin: druid::Vec2 = druid::Vec2::new(0.0, self.label_size.height);
            ctx.transform(druid::Affine::translate(layout_origin));

            let mut visible: druid::Region = ctx.region().clone();
            let mut size: Size = self.dropdown_size;
            size.height += self.label_size.height;

            visible.intersect_with(size.to_rect());
            visible -= layout_origin;

            ctx.with_child_ctx(
                druid::Region::from(druid::Rect::from_points(
                    druid::Point::ORIGIN,
                    self.dropdown_size.to_vec2().to_point(),
                )),
                |ctx| {
                    self.dropdown.paint(ctx, data, env);
                },
            );
        }
    }
}
