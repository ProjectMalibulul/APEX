package collab

import (
	"reflect"
	"testing"
)

func TestORSetAddReturnsValue(t *testing.T) {
	set := NewORSet()
	set.Add("alice", "op-1")

	got := set.Values()

	if !reflect.DeepEqual(got, []string{"alice"}) {
		t.Fatalf("Values() = %v, want [alice]", got)
	}
}

func TestORSetRemoveHidesObservedAdd(t *testing.T) {
	set := NewORSet()
	set.Add("alice", "op-1")
	set.Remove("alice")

	got := set.Values()

	if len(got) != 0 {
		t.Fatalf("Values() = %v, want empty", got)
	}
}

func TestORSetMergePreservesConcurrentAdd(t *testing.T) {
	left := NewORSet()
	right := NewORSet()
	left.Add("alice", "op-1")
	right.Add("alice", "op-2")
	left.Remove("alice")

	left.Merge(right)
	got := left.Values()

	if !reflect.DeepEqual(got, []string{"alice"}) {
		t.Fatalf("Values() = %v, want [alice]", got)
	}
}
