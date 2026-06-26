using Godot;

namespace ExampleProject
{
	[GlobalClass]
	public partial class EnemyAiConfig : Resource
	{
		[Export]
		public float WanderRadius { get; set; }

		[Export]
		public float RepickSecs { get; set; }

		[Export]
		public float StopDistance { get; set; }

		[Export]
		public float WalkSpeed { get; set; }

		[Export]
		public float FlySpeed { get; set; }

		[Export]
		public float FlyYMin { get; set; }

		[Export]
		public float FlyYMax { get; set; }

		[Export]
		public float FlyChance { get; set; }

		[Export]
		public float PlayAreaHalf { get; set; }

		[Export]
		public float MinWanderDistance { get; set; }

		[Export]
		public float TurnSpeedRad { get; set; }

		[Export]
		public float AnimCrossfadeSecs { get; set; }
	}

}
